use crate::cpu::{Cpu, StateType, CCR};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn bior_rn(&mut self, opcode: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_ccr(CCR::C, (!(value >> imm) & 1) | self.read_ccr(CCR::C));
        Ok(self.calc_state(StateType::I, 1)?)
    }

    pub(in super::super) fn bior_ern(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let register = Cpu::get_nibble_opcode(opcode, 3)?;
        let access_addr = self.get_addr_ern(register)?;
        let value = self.read_ern_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        self.write_ccr(CCR::C, (!(value >> imm) & 1) | self.read_ccr(CCR::C));
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, access_addr)?)
    }

    pub(in super::super) fn bior_abs(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
        let value = self.read_abs8_b(opcode as u8)?;
        self.write_ccr(CCR::C, (!(value >> imm) & 1) | self.read_ccr(CCR::C));
        let access_addr = self.get_addr_abs8(opcode as u8);
        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::L, 1, access_addr)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, memory::MEMORY_START_ADDR};

    #[test]
    fn test_bior_rn() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // bit7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x7f).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_rn_b(0xf, 0xfe).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x8f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
    }

    #[test]
    fn test_bior_ern() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x7f).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x74, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // register 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0xfe).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x70, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffcf20, 0x01).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0xfe).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffcf20, 0x01).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }

    #[test]
    fn test_bior_abs() {
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0xfe).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x7f).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 1;
        cpu.write_abs24_b(0xffff12, 0x01).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0xfe).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.pc = MEMORY_START_ADDR;
        cpu.ccr = 0;
        cpu.write_abs24_b(0xffff12, 0x01).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x80]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }
}
