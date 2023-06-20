use crate::cpu::{Cpu, CCR};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_b(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0xa0..=0xaf => return self.cmp_b_imm(opcode),
            0x1c => return self.cmp_b_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_b_proc(&mut self, dest: u8, src: u8) -> u8 {
        let (value, overflowed) = (dest as i8).overflowing_sub(src as i8);
        if (dest & 0x0f) + (!src & 0x0f) + 1 > 0x0f {
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

        if (dest as u16) + (!src as u16) + 1 > 0xff {
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
