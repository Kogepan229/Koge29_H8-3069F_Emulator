use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub fn sub_l_calc(&mut self, dest: u32, src: u32) -> u32 {
        let (result, overflowed) = (dest as i32).overflowing_sub(src as i32);

        self.change_ccr(CCR::H, (dest & 0x0fffffff) < (src & 0x0fffffff));
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, dest < src);

        result as u32
    }

    pub(in super::super) fn sub_l(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x7a => return self.sub_l_imm(opcode),
            0x1a => return self.sub_l_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    // 仕様書の図ではimmが16bitになっているが実際は32bit
    fn sub_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = (self.fetch() as u32) << 16 | self.fetch() as u32;
        let mut f = || -> Result<()> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_l(register)?;
            let result = self.sub_l_calc(dest, imm);
            self.write_rn_l(register, result)?;
            Ok(())
        };
        f().with_context(|| format!("imm(opcode2, 3) [{:x}]", imm))?;
        Ok(self.calc_state(StateType::I, 3)?)
    }

    fn sub_l_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_l(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
        let src = self.read_rn_l(register_src)?;
        let result = self.sub_l_calc(dest, src);
        self.write_rn_l(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
