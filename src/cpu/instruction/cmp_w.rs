use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub fn cmp_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = self.fetch();
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_w_calc(dest, imm);
        Ok(self.calc_state(StateType::I, 2)?)
    }

    pub fn cmp_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_w_calc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
