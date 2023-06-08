use crate::cpu::{Cpu, CCR};
use anyhow::Result;

impl<'a> Cpu<'a> {
    pub(in super::super) fn inc_b(&mut self, opcode: u16) -> Result<usize> {
        let result = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)? + 1;
        self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, result)?;
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
        if result == 0x80 {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }
        Ok(2)
    }

    pub(in super::super) fn inc_w_1(&mut self, opcode: u16) -> Result<usize> {
        let result = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)? + 1;
        self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, result)?;
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
        if result == 0x8000 {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }
        Ok(2)
    }

    pub(in super::super) fn inc_w_2(&mut self, opcode: u16) -> Result<usize> {
        let result = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)? + 1;
        self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, result)?;
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
        if result == 0x8000 || result == 0x8001 {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }
        Ok(2)
    }

    pub(in super::super) fn inc_l_1(&mut self, opcode: u16) -> Result<usize> {
        let result = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)? + 1;
        self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4)?, result)?;
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
        if result == 0x80000000 {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }
        Ok(2)
    }

    pub(in super::super) fn inc_l_2(&mut self, opcode: u16) -> Result<usize> {
        let result = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)? + 1;
        self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4)?, result)?;
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
        if result == 0x80000000 || result == 0x80000001 {
            self.write_ccr(CCR::V, 1);
        } else {
            self.write_ccr(CCR::V, 0);
        }
        Ok(2)
    }
}
