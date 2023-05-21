use super::super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn sub_b(&mut self, opcode: u16) -> Result<usize> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_b(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
        let src = self.read_rn_b(register_src)?;
        let result = self.sub_b_proc(dest, src);
        self.write_rn_b(register_dest, result)?;
        Ok(2)
    }

    fn sub_b_proc(&mut self, dest: u8, src: u8) -> u8 {
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
}
