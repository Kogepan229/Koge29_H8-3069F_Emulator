use super::super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn cmp_l(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x7a => return self.cmp_l_imm(opcode),
            0x1f => return self.cmp_l_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_l_proc(&mut self, dest: u32, src: u32) -> u32 {
        let (value, overflowed) = (dest as i32).overflowing_sub(src as i32);
        if (dest >> 27) & 1 == 1 && (value >> 27) & 1 == 0 {
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

        if (dest >> 31) & 1 == 1 && (value >> 31) & 1 == 0 {
            self.write_ccr(CCR::C, 1);
        } else {
            self.write_ccr(CCR::C, 0);
        }

        value as u32
    }

    fn cmp_l_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = (self.fetch() as u32) << 16 | self.fetch() as u32;
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_l_proc(dest, imm);
        Ok(6)
    }

    fn cmp_l_rn(&mut self, opcode: u16) -> Result<usize> {
        let src = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_l_proc(dest, src);
        Ok(2)
    }
}