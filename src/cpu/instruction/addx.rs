use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    fn addx_proc(&mut self, dest: u8, src: u8) -> u8 {
        let (value, overflowed) = (dest as i8).overflowing_add(src as i8);
        let (value, overflowed2) = (value).overflowing_add_unsigned(self.read_ccr(CCR::C));
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

        if overflowed || overflowed2 {
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

    pub(in super::super) fn addx_imm(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 2)?;
        let dest = self.read_rn_b(register)?;
        let result = self.addx_proc(dest, opcode as u8);
        self.write_rn_b(register, result)?;
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn addx_rn(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_b(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let src = self.read_rn_b(register_src)?;
        let result = self.addx_proc(dest, src);
        self.write_rn_b(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
