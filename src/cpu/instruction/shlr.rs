use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn shlr_b(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_b(register)?;
        let result = src >> 1;
        self.write_rn_b(register, result)?;
        self.write_ccr(CCR::N, 0);
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, src & 1);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn shlr_w(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_w(register)?;
        let result = src >> 1;
        self.write_rn_w(register, result)?;
        self.write_ccr(CCR::N, 0);
        if result == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
        self.write_ccr(CCR::C, (src & 1) as u8);
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn shlr_l(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let src = self.read_rn_l(register)?;
        let result = src >> 1;
        self.write_rn_l(register, result)?;
        self.write_ccr(CCR::N, 0);
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
    fn test_shlr_b() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x00]);
        cpu.write_rn_b(0, 0b0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0010_1101);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x0f]);
        cpu.write_rn_b(0xf, 0b0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_b(0xf).unwrap(), 0b0010_1101);

        // check CCR N, bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x00]);
        cpu.write_rn_b(0, 0b1010_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0101_0101);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x00]);
        cpu.write_rn_b(0, 0b0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0b0010_1010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x00]);
        cpu.write_rn_b(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_b(0).unwrap(), 0);
    }

    #[test]
    fn test_shlr_w() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x10]);
        cpu.write_rn_w(0, 0b0101_0101_0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0010_1010_1010_1101);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x1f]);
        cpu.write_rn_w(0xf, 0b0101_0101_0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0b0010_1010_1010_1101);

        // check CCR N, bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x10]);
        cpu.write_rn_w(0, 0b1010_1010_1010_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0101_0101_0101_0101);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x10]);
        cpu.write_rn_w(0, 0b0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0b0010_1010_1010_1010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x10]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[test]
    fn test_shlr_l() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x30]);
        cpu.write_rn_l(0, 0b0101_0101_0101_0101_0101_0101_0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b0010_1010_1010_1010_1010_1010_1010_1101);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x37]);
        cpu.write_rn_l(7, 0b0101_0101_0101_0101_0101_0101_0101_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0b0010_1010_1010_1010_1010_1010_1010_1101);

        // check CCR N, bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001111;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x30]);
        cpu.write_rn_l(0, 0b1010_1010_1010_1010_1010_1010_1010_1010).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000000);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b0101_0101_0101_0101_0101_0101_0101_0101);

        // check CCR C
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001110;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x30]);
        cpu.write_rn_l(0, 0b0101_0101_0101_0101_0101_0101_0101_0101).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000001);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0b0010_1010_1010_1010_1010_1010_1010_1010);

        // check CCR Z
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0b00001011;

        cpu.bus.memory[0..2].copy_from_slice(&[0x11, 0x30]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0b00000100);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
