use crate::cpu::{Cpu, StateType, ADDRESS_MASK};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn trapa(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = self.read_rn_l(7)? & ADDRESS_MASK;
        let imm = Cpu::get_nibble_opcode(opcode, 3)?;
        let vec_addr: u32 = (0x20 + 4 * imm).into();
        let dest_addr = self.read_abs24_l(vec_addr)?;

        self.write_dec_ern_l(7, ((self.ccr as u32) << 24) | self.pc)?;

        self.pc = dest_addr & ADDRESS_MASK;
        self.write_ccr(crate::cpu::CCR::I, 1);
        // TODO: set CCR::UI to 1 if it is used as interrupt mask bit.

        Ok(self.calc_state(StateType::I, 2)?
            + self.calc_state_with_addr(StateType::J, 2, dest_addr)?
            + self.calc_state_with_addr(StateType::K, 2, access_addr)?
            + self.calc_state(StateType::N, 4)?)
    }
}
