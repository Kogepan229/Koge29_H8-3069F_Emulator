use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn cmp_l(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x7a => return self.cmp_l_imm(opcode),
            0x1f => return self.cmp_l_rn(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn cmp_l_proc(&mut self, dest: u32, src: u32) -> u32 {
        let (result, overflowed) = dest.overflowing_sub(src);

        self.change_ccr(CCR::H, (dest & 0x0fffffff) < (src & 0x0fffffff));
        self.change_ccr(CCR::N, (result as i32) < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, dest < src);

        result
    }

    fn cmp_l_imm(&mut self, opcode: u16) -> Result<u8> {
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        let imm = (self.fetch() as u32) << 16 | self.fetch() as u32;
        self.cmp_l_proc(dest, imm);
        Ok(self.calc_state(StateType::I, 3)?)
    }

    fn cmp_l_rn(&mut self, opcode: u16) -> Result<u8> {
        let src = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)? & 0x7)?;
        let dest = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 4)?)?;
        self.cmp_l_proc(dest, src);
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
