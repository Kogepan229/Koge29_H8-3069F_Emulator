use super::super::*;
use anyhow::{bail, Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn mov_l(&mut self, opcode: u16) -> Result<usize> {
        if opcode & 0xff00 == 0x0f00 {
            return self.mov_l_rn(opcode);
        }
        if opcode & 0xfff8 == 0x7a00 {
            return self.mov_l_imm(opcode);
        }
        let opcode2 = self.fetch();
        match (opcode2 >> 8) as u8 {
            0x69 => return self.mov_l_ern(opcode2),
            0x6f => return self.mov_l_disp16(opcode2),
            0x78 => return self.mov_l_disp24(opcode2),
            0x6d => return self.mov_l_inc(opcode2),
            0x6b => return self.mov_l_abs(opcode2),
            _ => bail!("invalid opcode [{:>04x} {:>04x}]", opcode, opcode2),
        }
    }

    fn mov_l_proc_pcc(&mut self, src: u32) {
        if (src as i32) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if src == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
    }

    fn mov_l_rn(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)? & 0x07)?;
            self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_l_proc_pcc(value);
            return Ok(2);
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))
    }

    fn mov_l_imm(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let imm = (self.fetch() as u32) << 16 | self.fetch() as u32;
            self.write_rn_l((opcode & 0x000f) as u8, imm)?;
            self.mov_l_proc_pcc(imm);
            return Ok(6);
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))
    }

    fn mov_l_ern(&mut self, opcode2: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode2 & 0x0080 == 0 {
                let value = self.read_ern_l(Cpu::get_nibble_opcode(opcode2, 3)?)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_ern_l(Cpu::get_nibble_opcode(opcode2, 3)? & 0x07, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(8);
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))
    }

    fn mov_l_disp16(&mut self, opcode2: u16) -> Result<usize> {
        let disp = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode2 & 0x0080 == 0 {
                let value = self.read_disp16_ern_l(Cpu::get_nibble_opcode(opcode2, 3)?, disp)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_disp16_ern_l(Cpu::get_nibble_opcode(opcode2, 3)? & 0x07, disp, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(10);
        };
        f().with_context(|| format!("opcode2 [{:x}] disp [{:x}]", opcode2, disp))
    }

    fn mov_l_disp24(&mut self, opcode2: u16) -> Result<usize> {
        let opcode3 = self.fetch();
        let disp = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode2 & 0x0080 == 0 {
                let value = self.read_disp24_ern_l(Cpu::get_nibble_opcode(opcode2, 3)?, disp)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode3, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode3, 4)?)?;
                self.write_disp24_ern_l(Cpu::get_nibble_opcode(opcode2, 3)? & 0x07, disp, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(14);
        };
        f().with_context(|| {
            format!(
                "opcode2 [{:x}] opcode3 [{:x}] disp [{:x}]",
                opcode2, opcode3, disp
            )
        })
    }

    fn mov_l_inc(&mut self, opcode2: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode2 & 0x0080 == 0 {
                let value = self.read_inc_ern_l(Cpu::get_nibble_opcode(opcode2, 3)?)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_inc_ern_l(Cpu::get_nibble_opcode(opcode2, 3)? & 0x07, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(10);
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))
    }

    fn mov_l_abs(&mut self, opcode2: u16) -> Result<usize> {
        match opcode2 & 0xfff0 {
            0x6b00 | 0x6b80 => return self.mov_l_abs16(opcode2),
            0x6b20 | 0x6ba0 => return self.mov_l_abs24(opcode2),
            _ => bail!("invalid opcode2 [{:x}]", opcode2),
        }
    }

    fn mov_l_abs16(&mut self, opcode2: u16) -> Result<usize> {
        let abs_addr = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode2 & 0xfff0 == 0x6b00 {
                let value = self.read_abs16_l(abs_addr)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_abs16_l(abs_addr, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(10);
        };
        f().with_context(|| format!("opcode2 [{:x}] abs_addr [{:x}]", opcode2, abs_addr))
    }

    fn mov_l_abs24(&mut self, opcode2: u16) -> Result<usize> {
        let abs_addr = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode2 & 0xfff0 == 0x6b20 {
                let value = self.read_abs24_l(abs_addr)?;
                self.write_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_l_proc_pcc(value);
            } else {
                let value = self.read_rn_l(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_abs24_l(abs_addr, value)?;
                self.mov_l_proc_pcc(value);
            }
            return Ok(12);
        };
        f().with_context(|| format!("opcode2 [{:x}] abs_addr [{:x}]", opcode2, abs_addr))
    }
}
