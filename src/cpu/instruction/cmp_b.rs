use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub fn cmp_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 2)?;
        let rd = self.read_rn_b(rd_i)?;
        let imm = opcode as u8;
        self.sub_b_calc(rd, imm);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub fn cmp_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 3)?)?;
        let dest = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_b_calc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
