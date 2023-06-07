use crate::cpu::{Cpu, CCR};
use anyhow::{bail, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn cmp_b(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0xa0..=0xa7 => return self.cmp_b_imm(opcode),
            0x1c => return self.cmp_b_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_b_proc(&mut self, dest: u8, src: u8) -> u8 {
        let (value, overflowed) = (dest as i8).overflowing_sub(src as i8);
        if (dest >> 3) & 1 == 1 && (value >> 3) & 1 == 0 {
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

        if (dest >> 7) & 1 == 1 && (value >> 7) & 1 == 0 {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u8
    }

    fn cmp_b_imm(&mut self, opcode: u16) -> Result<usize> {
        let dest = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_b_proc(dest, opcode as u8);
        Ok(2)
    }

    fn cmp_b_rn(&mut self, opcode: u16) -> Result<usize> {
        let src = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_b_proc(dest, src);
        Ok(2)
    }
}