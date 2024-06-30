use crate::cpu::{Cpu, StateType};
use anyhow::{bail, Result};

impl Cpu {
    pub(in super::super)  fn jmp(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x59 => return self.jmp_ern(opcode),
            0x5a => return self.jmp_abs(opcode),
            0x5b => return self.jmp_indirect(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

     fn jmp_ern(&mut self, opcode: u16) -> Result<u8> {
        let addr = self.read_rn_l(Cpu::get_nibble_opcode(opcode, 3)?)?;
        self.pc = addr;
        Ok(self.calc_state(StateType::I, 2)?)
    }
     fn jmp_abs(&mut self, opcode: u16) -> Result<u8> {
        let abs_addr = ((opcode & 0x00ff) as u32) << 16 | self.fetch() as u32;
        self.pc = abs_addr;
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }
     fn jmp_indirect(&mut self, opcode: u16) -> Result<u8> {
        let addr = self.read_abs8_l(opcode as u8)?;
        self.pc = addr;
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2)?
            + self
                .calc_state_with_addr(StateType::J, 1, access_addr)
                ?
            + self.calc_state(StateType::N, 2)?)
    }
}
