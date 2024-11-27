use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_w(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x79 => return self.cmp_w_imm(opcode),
            0x1d => return self.cmp_w_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = self.fetch();
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_w_calc(dest, imm);
        Ok(self.calc_state(StateType::I, 2)?)
    }

    fn cmp_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_w_calc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
