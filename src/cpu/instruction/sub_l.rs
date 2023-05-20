use super::super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn sub_l(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x7a => return self.sub_l_imm(opcode),
            0x1a => return self.sub_l_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn sub_l_proc(&mut self, dest: u32, src: u32) -> u32 {
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

    fn sub_l_imm(&mut self, opcode: u16) -> Result<usize> {
        let opcode2 = self.fetch();
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_l(register)?;
            let result = self.sub_l_proc(dest, opcode2 as u32);
            self.write_rn_l(register, result)?;
            Ok(6)
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))?;
    }

    fn sub_l_rn(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
            let dest = self.read_rn_l(register_dest)?;
            let register_src = Cpu::get_nibble_opcode(opcode, 3)? & 0x7;
            let src = self.read_rn_l(register_src)?;
            let result = self.sub_l_proc(dest, src);
            self.write_rn_l(register_dest, result)?;
            Ok(2)
        };
        f()
    }
}
