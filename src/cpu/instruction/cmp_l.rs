use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_l(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x7a => return self.cmp_l_imm(opcode),
            0x1f => return self.cmp_l_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        let opcode2 = self.fetch();
        let opcode3 = self.fetch();
        let imm = ((opcode2 as u32) << 16) | opcode3 as u32;
        self.sub_l_calc(dest, imm);

        Ok(self.calc_state(StateType::I, 3)?)
    }

    fn cmp_l_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.sub_l_calc(dest, src);

        Ok(self.calc_state(StateType::I, 1)?)
    }
}
