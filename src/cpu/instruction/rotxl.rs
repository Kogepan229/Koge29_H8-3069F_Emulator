use crate::cpu::{Cpu, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn rotxl_b(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        let result = (src << 1) | (self.ccr & 1);
        self.write_rn_b(register, result)?;
        if result & 0x80 == 0x80 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, (src >> 7) & 1);
        Ok(2)
    }

    pub(in super::super) fn rotxl_w(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        let result = (src << 1) | ((self.ccr as u16) & 1);
        self.write_rn_w(register, result)?;
        if result & 0x8000 == 0x8000 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, ((src >> 15) & 1) as u8);
        Ok(2)
    }

    pub(in super::super) fn rotxl_l(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        let result = (src << 1) | ((self.ccr as u32) & 1);
        self.write_rn_l(register, result)?;
        if result & 0x8000_0000 == 0x8000_0000 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, ((src >> 31) & 1) as u8);
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_rotxl_b() {
        // check CCR N, V, value
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x00]);
        cpu.write_rn_b(0, 0b0101_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b1010_1001);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x0f]);
        cpu.write_rn_b(0xf, 0b0101_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b1010_1001);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x00]);
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x00]);
        cpu.write_rn_b(0, 0b1001_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0010_1000);
    }

    #[test]
    fn test_rotxl_w() {
        // check CCR N, V, value
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x10]);
        cpu.write_rn_w(0, 0b0101_0101_0101_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b1010_1010_1010_1001);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x1f]);
        cpu.write_rn_w(0xf, 0b0101_0101_0101_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b1010_1010_1010_1001);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x10]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x10]);
        cpu.write_rn_w(0, 0b1001_0101_0101_0100).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0010_1010_1010_1000);
    }

    #[test]
    fn test_rotxl_l() {
        // check CCR N, V, value
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x30]);
        cpu.write_rn_l(0, 0b0101_0101_0101_0101_0101_0101_0101_0100)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1001
        );

        // check register 7
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00000111;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x37]);
        cpu.write_rn_l(7, 0b0101_0101_0101_0101_0101_0101_0101_0100)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001000);
        assert_eq!(
            cpu.read_rn_l(7).unwrap(),
            0b1010_1010_1010_1010_1010_1010_1010_1001
        );

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001010;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x30]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.ccr = 0b00001110;

        cpu.mcu.memory[0..2].copy_from_slice(&[0x12, 0x30]);
        cpu.write_rn_l(0, 0b1001_0101_0101_0101_0101_0101_0101_0100)
            .unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(
            cpu.read_rn_l(0).unwrap(),
            0b0010_1010_1010_1010_1010_1010_1010_1000
        );
    }
}
