use crate::cpu::{Cpu, CCR};
use anyhow::{Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn and_b_imm(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 2)?;
        let result = self.read_rn_b(register)? & opcode as u8;
        self.write_rn_b(register, result)?;
        if (result as i8) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if (result as i8) == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(2)
    }

    pub(in super::super) fn and_b_rn(&mut self, opcode: u16) -> Result<usize> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_b(register_src)? & self.read_rn_b(register_dest)?;
        self.write_rn_b(register_dest, result)?;
        if (result as i8) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(2)
    }

    pub(in super::super) fn and_w_imm(&mut self, opcode: u16) -> Result<usize> {
        let opcode2 = self.fetch();

        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_w(register)? & opcode2;
            self.write_rn_w(register, result)?;
            if (result as i16) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(4);
        };
        f().with_context(|| format!("opcode2(imm) [{:x}]", opcode2))
    }

    pub(in super::super) fn and_w_rn(&mut self, opcode: u16) -> Result<usize> {
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let result = self.read_rn_w(register_src)? & self.read_rn_w(register_dest)?;
        self.write_rn_w(register_dest, result)?;
        if (result as i16) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        Ok(2)
    }

    pub(in super::super) fn and_l_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = ((self.fetch() as u32) << 16) | self.fetch() as u32;

        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 4)?;
            let result = self.read_rn_l(register)? & imm;
            self.write_rn_l(register, result)?;
            if (result as i32) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(6);
        };
        f().with_context(|| format!("imm(opcode2, 3) [{:x}]", imm))
    }

    pub(in super::super) fn and_l_rn(&mut self, _opcode: u16, opcode2: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register_src = Cpu::get_nibble_opcode(opcode2, 3)?;
            let register_dest = Cpu::get_nibble_opcode(opcode2, 4)?;
            let result = self.read_rn_l(register_src)? & self.read_rn_l(register_dest)?;
            self.write_rn_l(register_dest, result)?;
            if (result as i32) < 0 {
                self.write_ccr(CCR::N, 1);
            } else {
                self.write_ccr(CCR::N, 0);
            }
            if result == 0 {
                self.write_ccr(CCR::Z, 1);
            } else {
                self.write_ccr(CCR::Z, 0);
            }
            self.write_ccr(CCR::V, 0);
            return Ok(4);
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))
    }
}
