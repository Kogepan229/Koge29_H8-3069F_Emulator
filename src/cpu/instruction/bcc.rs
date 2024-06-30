use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super)  fn bcc(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x40 => return self.bra8(opcode),
            0x41 => return self.brn8(),
            0x42 => return self.bhi8(opcode),
            0x43 => return self.bls8(opcode),
            0x44 => return self.bcc8(opcode),
            0x45 => return self.bcs8(opcode),
            0x46 => return self.bne8(opcode),
            0x47 => return self.beq8(opcode),
            0x48 => return self.bvc8(opcode),
            0x49 => return self.bvs8(opcode),
            0x4a => return self.bpl8(opcode),
            0x4b => return self.bmi8(opcode),
            0x4c => return self.bge8(opcode),
            0x4d => return self.blt8(opcode),
            0x4e => return self.bgt8(opcode),
            0x4f => return self.ble8(opcode),
            0x58 => match opcode as u8 {
                0x00 => return self.bra16(),
                0x10 => return self.brn16(),
                0x20 => return self.bhi16(),
                0x30 => return self.bls16(),
                0x40 => return self.bcc16(),
                0x50 => return self.bcs16(),
                0x60 => return self.bne16(),
                0x70 => return self.beq16(),
                0x80 => return self.bvc16(),
                0x90 => return self.bvs16(),
                0xa0 => return self.bpl16(),
                0xb0 => return self.bmi16(),
                0xc0 => return self.bge16(),
                0xd0 => return self.blt16(),
                0xe0 => return self.bgt16(),
                0xf0 => return self.ble16(),
                _ => bail!("invalid opcode [{:>04x}]", opcode),
            },
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

     fn bra8(&mut self, opcode: u16) -> Result<u8> {
        self.pc_disp8(opcode as u8)?;
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bra16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        self.pc_disp16(opcode2)
            .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn brn8(&mut self) -> Result<u8> {
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn brn16(&mut self) -> Result<u8> {
        let _opcode2 = self.fetch();
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bhi8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bhi16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bls8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bls16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bcc8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bcc16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::C) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bcs8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::C) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bcs16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::C) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bne8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bne16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::Z) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn beq8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn beq16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::Z) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bvc8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::V) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bvc16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::V) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bvs8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::V) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bvs16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::V) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bpl8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::N) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bpl16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::N) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bmi8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::N) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bmi16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::N) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bge8(&mut self, opcode: u16) -> Result<u8> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bge16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn blt8(&mut self, opcode: u16) -> Result<u8> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn blt16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn bgt8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn bgt16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 0 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }

     fn ble8(&mut self, opcode: u16) -> Result<u8> {
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(self.calc_state(StateType::I, 2)?)
    }

     fn ble16(&mut self) -> Result<u8> {
        let opcode2 = self.fetch();
        if self.read_ccr(CCR::Z) | (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) == 1 {
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state(StateType::N, 2)?)
    }
}
