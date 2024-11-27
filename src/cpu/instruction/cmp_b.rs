use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_b(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0xa0..=0xaf => return self.cmp_b_imm(opcode),
            0x1c => return self.cmp_b_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 2)?;
        let rd = self.read_rn_b(rd_i)?;
        let imm = opcode as u8;
        self.sub_b_calc(rd, imm);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    fn cmp_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 3)?)?;
        let dest = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_b_calc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
