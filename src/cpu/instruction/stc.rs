use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn stc_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        self.write_rn_b(rd_i, self.ccr)?;

        self.calc_state(StateType::I, 1)
    }

    pub(in super::super) fn stc_w_ern(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        self.write_ern_w(erd_i, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::M, 1)?)
    }

    pub(in super::super) fn stc_w_disp16(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        let disp = self.fetch();
        self.write_disp16_ern_w(erd_i, disp, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 3)? + self.calc_state(StateType::M, 1)?)
    }

    pub(in super::super) fn stc_w_disp24(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)?;
        self.fetch(); // opcode3
        let opcode4 = self.fetch();
        let opcode5 = self.fetch();
        let disp = (u32::from(opcode4) << 16) | u32::from(opcode5);
        self.write_disp24_ern_w(erd_i, disp, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 5)? + self.calc_state(StateType::M, 1)?)
    }

    pub(in super::super) fn stc_w_inc_ern(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        self.write_inc_ern_w(erd_i, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::M, 1)? + self.calc_state(StateType::N, 2)?)
    }

    pub(in super::super) fn stc_abs16(&mut self) -> Result<u8> {
        let addr = self.fetch();
        self.write_abs16_w(addr, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 3)? + self.calc_state(StateType::M, 1)?)
    }

    pub(in super::super) fn stc_abs24(&mut self) -> Result<u8> {
        let opcode3 = self.fetch();
        let opcode4 = self.fetch();
        let addr = (u32::from(opcode3) << 16) | u32::from(opcode4);
        self.write_abs24_w(addr, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 4)? + self.calc_state(StateType::M, 1)?)
    }
}
