use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn divxu_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;

        let rd = self.read_rn_w(rd_i)?;
        let rs = self.read_rn_b(rs_i)?;

        if (rs as i8) < 0 {
            self.write_ccr(crate::cpu::CCR::N, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::N, 0);
        }

        if rs == 0 {
            self.write_ccr(crate::cpu::CCR::Z, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::Z, 0);
        }

        let quotient: u16 = if rs == 0 { 0 } else { rd / u16::from(rs) };
        let remainder: u16 = if rd == 0 { 0 } else { rd % u16::from(rs) };
        let result = (remainder << 8) | (quotient & 0xff);
        self.write_rn_w(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 12)?)
    }

    pub(in super::super) fn divxu_w(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)? & 0b111;
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;

        let rd = self.read_rn_l(rd_i)?;
        let rs = self.read_rn_w(rs_i)?;

        if (rs as i16) < 0 {
            self.write_ccr(crate::cpu::CCR::N, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::N, 0);
        }

        if rs == 0 {
            self.write_ccr(crate::cpu::CCR::Z, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::Z, 0);
        }

        let quotient: u32 = if rs == 0 { 0 } else { rd / u32::from(rs) };
        let remainder: u32 = if rd == 0 { 0 } else { rd % u32::from(rs) };
        let result = (remainder << 16) | (quotient & 0xffff);
        self.write_rn_l(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 20)?)
    }
}
