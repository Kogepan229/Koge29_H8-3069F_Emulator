use crate::cpu::{Cpu, StateType, ADDRESS_MASK};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn rte(&mut self) -> Result<u8> {
        let access_addr = self.read_rn_l(7)? & ADDRESS_MASK;
        let ccr_pc = self.read_inc_ern_l(7)?;
        self.ccr = (ccr_pc >> 24) as u8;
        self.pc = ccr_pc & ADDRESS_MASK;

        Ok(self.calc_state(StateType::I, 2)?
            + self.calc_state_with_addr(StateType::K, 2, access_addr)?
            + self.calc_state(StateType::N, 2)?)
    }
}
