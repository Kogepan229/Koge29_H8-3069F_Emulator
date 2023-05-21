use super::super::*;
use anyhow::{bail, Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn mov_w(&mut self, opcode: u16) -> Result<usize> {
        match opcode as u8 {
            0x0d => return self.mov_w_rn(opcode),
            0x79 => return self.mov_w_imm(opcode),
            0x69 => return self.mov_w_ern(opcode),
            0x6f => return self.mov_w_disp16(opcode),
            0x78 => return self.mov_w_disp24(opcode),
            0x6d => return self.mov_w_inc_or_dec(opcode),
            0x6b => return self.mov_w_abs(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn mov_w_proc_pcc(&mut self, src: u16) {
        if (src as i16) < 0 {
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

    fn mov_w_rn(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
            self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_w_proc_pcc(value);
            return Ok(2);
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))
    }

    fn mov_w_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = self.fetch();
        let mut f = || -> Result<usize> {
            self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, imm)?;
            self.mov_w_proc_pcc(imm);
            return Ok(4);
        };
        f().with_context(|| format!("opcode [{:x}] imm(opcode2) [{:x}]", opcode, imm))
    }

    fn mov_w_ern(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_ern_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(4);
        };
        f()
    }

    fn mov_w_disp16(&mut self, opcode: u16) -> Result<usize> {
        let disp = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_disp16_ern_w(Cpu::get_nibble_opcode(opcode, 3)?, disp)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_disp16_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, disp, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f().with_context(|| format!("disp [{:x}]", disp))
    }

    fn mov_w_disp24(&mut self, opcode: u16) -> Result<usize> {
        let opcode2 = self.fetch();
        let disp = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode2 & 0xfff0 == 0x6b20 {
                let value = self.read_disp24_ern_w(Cpu::get_nibble_opcode(opcode, 3)?, disp)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_disp24_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, disp, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(10);
        };
        f().with_context(|| format!("opcode2 [{:x}] disp [{:x}]", opcode2, disp))
    }

    fn mov_w_inc_or_dec(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_inc_ern_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_dec_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f()
    }

    fn mov_w_abs(&mut self, opcode: u16) -> Result<usize> {
        match opcode & 0xfff0 {
            0x6b00 | 0x6b80 => return self.mov_w_abs16(opcode),
            0x6b20 | 0x6ba0 => return self.mov_w_abs24(opcode),
            _ => bail!("invalid opcode [{:x}]", opcode),
        }
    }

    fn mov_w_abs16(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode & 0xfff0 == 0x6b00 {
                let value = self.read_abs16_w(abs_addr)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_abs16_w(abs_addr, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f().with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }

    fn mov_w_abs24(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode & 0xfff0 == 0x6b20 {
                let value = self.read_abs24_w(abs_addr)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_abs24_w(abs_addr, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(8);
        };
        f().with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }
}
