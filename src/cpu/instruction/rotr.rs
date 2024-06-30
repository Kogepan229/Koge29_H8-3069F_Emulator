use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn rotr_b(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        let result = (src >> 1) | (src << 7);
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
        self.write_ccr(CCR::C, src & 1);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn rotr_w(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        let result = (src >> 1) | (src << 15);
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
        self.write_ccr(CCR::C, (src & 1) as u8);

        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn rotr_l(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        let result = (src >> 1) | (src << 31);
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
        self.write_ccr(CCR::C, (src & 1) as u8);

        Ok(self.calc_state(StateType::I, 1)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[test]
    fn test_rotr_b() {
        // check value, CCR V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x80]);
        cpu.write_rn_b(0, 0b1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0101_0001);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x8f]);
        cpu.write_rn_b(0xf, 0b1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b0101_0001);

        // check CCR N, C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x80]);
        cpu.write_rn_b(0, 0b0100_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001001);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b1010_0010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x80]);
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0);
    }

    #[test]
    fn test_rotr_w() {
        // check value, CCR V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x90]);
        cpu.write_rn_w(0, 0b1000_1010_1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0100_0101_0101_0001);

        // check register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x9f]);
        cpu.write_rn_w(0xf, 0b1000_1010_1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b0100_0101_0101_0001);

        // check CCR N, C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x90]);
        cpu.write_rn_w(0, 0b0001_0101_0101_0001).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b1000_1010_1010_1000);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0x90]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0);
    }

    #[test]
    fn test_rotr_l() {
        // check value, CCR V
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0xb0]);
        cpu.write_rn_l(0, 0b1000_1010_1010_1010_1010_1010_1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b0100_0101_0101_0101_0101_0101_0101_0001);

        // check register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0xb7]);
        cpu.write_rn_l(7, 0b1000_1010_1010_1010_1010_1010_1010_0010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0b0100_0101_0101_0101_0101_0101_0101_0001);

        // check CCR N, C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00000110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0xb0]);
        cpu.write_rn_l(0, 0b0001_0101_0101_0101_0101_0101_0101_0001).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00001001);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b1000_1010_1010_1010_1010_1010_1010_1000);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x13, 0xb0]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b0);
    }
}
