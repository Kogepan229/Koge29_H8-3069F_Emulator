use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    fn neg_b_proc(&mut self, value: u8) -> u8 {
        let (result, overflowed) = 0u8.overflowing_sub(value);

        self.change_ccr(CCR::H, (!value & 0x0f) + 1 > 0x0f);
        self.change_ccr(CCR::N, (result as i8) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, (!value as u16) + 1 > 0xff);

        result
    }

    pub(in super::super) fn neg_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_b(rd_i)?;

        let result = self.neg_b_proc(rd);
        self.write_rn_b(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }

    fn neg_w_proc(&mut self, value: u16) -> u16 {
        let (result, overflowed) = 0u16.overflowing_sub(value);

        self.change_ccr(CCR::H, (!value & 0x0fff) + 1 > 0x0fff);
        self.change_ccr(CCR::N, (result as i16) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, (!value as u32) + 1 > 0xffff);

        result
    }

    pub(in super::super) fn neg_w(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_w(rd_i)?;

        let result = self.neg_w_proc(rd);
        self.write_rn_w(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }

    fn neg_l_proc(&mut self, value: u32) -> u32 {
        let (result, overflowed) = 0u32.overflowing_sub(value);

        self.change_ccr(CCR::H, (!value & 0x0fffffff) + 1 > 0x0fffffff);
        self.change_ccr(CCR::N, (result as i32) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, (!value as u64) + 1 > 0xffffffff);

        result
    }

    pub(in super::super) fn neg_l(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rd = self.read_rn_l(rd_i)?;

        let result = self.neg_l_proc(rd);
        self.write_rn_l(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)?)
    }
}
