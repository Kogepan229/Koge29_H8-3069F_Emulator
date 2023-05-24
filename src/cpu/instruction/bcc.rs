use super::super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn bcc(&mut self, opcode: u16) -> Result<usize> {
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

    fn bra8(&mut self, opcode: u16) -> Result<usize> {
        self.pc_disp8(opcode as u8)?;
        Ok(4)
    }

    fn bra16(&mut self) -> Result<usize> {
        let opcode2 = self.fetch();
        self.pc_disp16(opcode2)
            .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        Ok(6)
    }

    fn brn8(&mut self) -> Result<usize> {
        Ok(4)
    }

    fn brn16(&mut self) -> Result<usize> {
        let _opcode2 = self.fetch();
        Ok(6)
    }

    fn bhi8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bhi16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bls8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bls16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::C) | self.read_ccr(CCR::Z) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bcc8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::C) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bcc16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::C) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bcs8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::C) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bcs16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::C) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bne8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::Z) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bne16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::Z) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn beq8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::Z) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn beq16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::Z) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bvc8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::V) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bvc16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::V) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bvs8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::V) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bvs16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::V) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bpl8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::N) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bpl16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::N) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bmi8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::N) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bmi16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::N) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bge8(&mut self, opcode: u16) -> Result<usize> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1 == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bge16(&mut self) -> Result<usize> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1 == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn blt8(&mut self, opcode: u16) -> Result<usize> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1 == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn blt16(&mut self) -> Result<usize> {
        if (self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1 == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn bgt8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::Z) | ((self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1) == 0 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn bgt16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::Z) | ((self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1) == 0 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }

    fn ble8(&mut self, opcode: u16) -> Result<usize> {
        if self.read_ccr(CCR::Z) | ((self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1) == 1 {
            self.pc_disp8(opcode as u8)?;
        }
        Ok(4)
    }

    fn ble16(&mut self) -> Result<usize> {
        if self.read_ccr(CCR::Z) | ((self.read_ccr(CCR::N) ^ self.read_ccr(CCR::V)) & 1) == 1 {
            let opcode2 = self.fetch();
            self.pc_disp16(opcode2)
                .with_context(|| format!("opcode2 [{:x}]", opcode2))?;
        }
        Ok(6)
    }
}
