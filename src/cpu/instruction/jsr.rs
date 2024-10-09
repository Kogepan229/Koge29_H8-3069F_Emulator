use crate::cpu::{Cpu, StateType, ADDRESS_MASK};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super) fn jsr(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x5d => return self.jsr_ern(opcode),
            0x5e => return self.jsr_abs(opcode),
            0x5f => return self.jsr_indirect(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn jsr_ern(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & ADDRESS_MASK;
        self.write_dec_ern_l(7, self.pc)?;
        let addr = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)?)?;
        self.pc = addr;
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::K, 2, access_addr)?)
    }

    fn jsr_abs(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = (self.read_rn_l(7)? - 4) & ADDRESS_MASK;
        let opcode2 = self.fetch();
        let abs_addr = (((opcode & 0x00ff) as u32) << 16) | opcode2 as u32;
        self.write_dec_ern_l(7, self.pc)?;
        self.pc = abs_addr;
        Ok(self.calc_state(StateType::I, 2)?
            + self.calc_state_with_addr(StateType::K, 2, access_addr)?
            + self.calc_state(StateType::N, 2)?)
    }

    fn jsr_indirect(&mut self, opcode: u16) -> Result<u8> {
        let abs_addr = (opcode & 0x00ff) as u8;
        self.write_dec_ern_l(7, self.pc)?;
        let addr = self.read_abs8_l(abs_addr)?;
        self.pc = addr;
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2)?
            + self.calc_state_with_addr(StateType::J, 2, access_addr)?
            + self.calc_state(StateType::K, 2)?)
    }
}
