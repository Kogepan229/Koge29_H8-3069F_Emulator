use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_w(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x79 => return self.cmp_w_imm(opcode),
            0x1d => return self.cmp_w_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_w_proc(&mut self, dest: u16, src: u16) -> u16 {
        let (result, overflowed) = dest.overflowing_sub(src);

        self.change_ccr(CCR::H, (dest & 0x0fff) < (src & 0x0fff));
        self.change_ccr(CCR::N, (result as i16) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, dest < src);

        result
    }

    fn cmp_w_imm(&mut self, opcode: u16) -> Result<u8> {
        let imm = self.fetch();
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_w_proc(dest, imm);
        Ok(self.calc_state(StateType::I, 2)?)
    }

    fn cmp_w_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
        let dest = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_w_proc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
