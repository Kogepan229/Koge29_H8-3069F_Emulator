use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub fn sub_b_calc(&mut self, dest: u8, src: u8) -> u8 {
        let (result, overflowed) = (dest as i8).overflowing_sub(src as i8);

        self.change_ccr(CCR::H, (dest & 0x0f) < (src & 0x0f));
        self.change_ccr(CCR::N, result < 0);
        self.change_ccr(CCR::Z, result == 0);
        self.change_ccr(CCR::V, overflowed);
        self.change_ccr(CCR::C, dest < src);

        result as u8
    }

    pub(in super::super) fn sub_b(&mut self, opcode: u16) -> Result<u8> {
        let register_dest = Cpu::get_nibble_opcode(opcode, 4)?;
        let dest = self.read_rn_b(register_dest)?;
        let register_src = Cpu::get_nibble_opcode(opcode, 3)?;
        let src = self.read_rn_b(register_src)?;
        let result = self.sub_b_calc(dest, src);
        self.write_rn_b(register_dest, result)?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
}
