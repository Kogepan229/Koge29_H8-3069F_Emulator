use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn dec_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(rd_i)? as i8;
        let (result, overflowed) = src.overflowing_sub(1);

        self.write_rn_b(rd_i, result as u8)?;
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn dec_w_1(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(rd_i)? as i16;
        let (result, overflowed) = src.overflowing_sub(1);

        self.write_rn_w(rd_i, result as u16)?;
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn dec_w_2(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(rd_i)? as i16;
        let (result, overflowed) = src.overflowing_sub(2);

        self.write_rn_w(rd_i, result as u16)?;
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn dec_l_1(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(rd_i)? as i32;
        let (result, overflowed) = src.overflowing_sub(1);
        self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4)?, result as u32)?;
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn dec_l_2(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(rd_i)? as i32;
        let (result, overflowed) = src.overflowing_sub(2);
        self.write_rn_l(Cpu::get_nibble_opcode(opcode, 4)?, result as u32)?;
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
