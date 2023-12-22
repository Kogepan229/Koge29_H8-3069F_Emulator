use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) async fn add_b(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x80..=0x8f => return self.add_b_imm(opcode).await,
            0x08 => return self.add_b_rn(opcode).await,
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn add_b_proc(&mut self, dest: u8, src: u8) -> u8 {
        let (value, overflowed) = (dest as i8).overflowing_add(src as i8);
        if (dest & 0x0f) + (src & 0x0f) > 0x0f {
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

        if (dest as u16) + (src as u16) > 0xff {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u8
    }

    async fn add_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 2)?;
        let dest = self.read_rn_b(register)?;
        let result = self.add_b_proc(dest, opcode as u8);
        self.write_rn_b(register, result)?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    async fn add_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_b(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
        let src = self.read_rn_b(register_src)?;
        let result = self.add_b_proc(dest, src);
        self.write_rn_b(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }
}
