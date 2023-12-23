use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super) async fn sub_l(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x7a => return self.sub_l_imm(opcode).await,
            0x1a => return self.sub_l_rn(opcode).await,
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn sub_l_proc(&mut self, dest: u32, src: u32) -> u32 {
        let (value, overflowed) = (dest as i32).overflowing_sub(src as i32);
        if (dest & 0x0fffffff) + (!src & 0x0fffffff) + 1 > 0x0fffffff {
            self.write_ccr(CCR::H, 1);
        } else {
            self.write_ccr(CCR::H, 0);
        }

        if value < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }

        if value == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }

        if overflowed {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }

        if (dest as u64) + (!src as u64) + 1 > 0xffffffff {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u32
    }

    // 仕様書の図ではimmが16bitになっているが実際は32bit
    async fn sub_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = (self.fetch().await as u32) << 16 | self.fetch().await as u32;
        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_l(register)?;
            let result = self.sub_l_proc(dest, imm);
            self.write_rn_l(register, result)?;
            Ok(())
        };
        f().with_context(|| format!("imm(opcode2, 3) [{:x}]", imm));
        Ok(self.calc_state(StateType::I, 3).await?)
    }

    async fn sub_l_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_l(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
        let src = self.read_rn_l(register_src)?;
        let result = self.sub_l_proc(dest, src);
        self.write_rn_l(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }
}
