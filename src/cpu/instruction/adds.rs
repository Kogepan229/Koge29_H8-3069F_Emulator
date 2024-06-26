use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn adds1(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_l(register)?;
        self.write_rn_l(register, value.wrapping_add(1))?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
    pub(in super::super) fn adds2(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_l(register)?;
        self.write_rn_l(register, value.wrapping_add(2))?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
    pub(in super::super) fn adds4(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_l(register)?;
        self.write_rn_l(register, value.wrapping_add(4))?;
        Ok(self.calc_state(StateType::I, 1)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[test]
    fn test_adds1() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 11);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x07]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 11);

        // overflow
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 1);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 1);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x00]);
        cpu.write_rn_l(0, -1i32 as u32).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[test]
    fn test_adds2() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 12);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x87]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 12);

        // overflow
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 1);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 2);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 2);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x80]);
        cpu.write_rn_l(0, -2i32 as u32).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }

    #[test]
    fn test_adds4() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 14);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x97]);
        cpu.write_rn_l(7, 10).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 14);

        // overflow
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, std::u32::MAX).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 3);

        // u32 MIN + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, std::u32::MIN).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), std::u32::MIN + 4);

        // 0 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 4);

        // -1 + 1
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.bus.memory[0..2].copy_from_slice(&[0x0b, 0x90]);
        cpu.write_rn_l(0, -4i32 as u32).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0);
    }
}
