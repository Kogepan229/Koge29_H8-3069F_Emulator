use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub fn cmp_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        let opcode2 = self.fetch();
        let opcode3 = self.fetch();
        let imm = ((opcode2 as u32) << 16) | opcode3 as u32;
        self.sub_l_calc(dest, imm);

        Ok(self.calc_state(StateType::I, 3)?)
    }

    pub fn cmp_l_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_l_calc(dest, src);

        Ok(self.calc_state(StateType::I, 1)?)
    }
}
