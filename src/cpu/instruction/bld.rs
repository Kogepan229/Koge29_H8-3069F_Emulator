use crate::cpu::{Cpu, CCR};
use anyhow::{Context as _, Result};

impl Cpu {
    pub(in super::super) fn bld_rn(&mut self, opcode: u16) -> Result<usize> {
        let register = Cpu::get_nibble_opcode(opcode, 4)?;
        let value = self.read_rn_b(register)?;
        let imm = Cpu::get_nibble_opcode(opcode, 3)? & 7;
        self.write_ccr(CCR::C, (value >> imm) & 1);
        Ok(2)
    }

    pub(in super::super) fn bld_ern(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let register = Cpu::get_nibble_opcode(opcode, 3)?;
            let value = self.read_ern_b(register)?;
            let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
            self.write_ccr(CCR::C, (value >> imm) & 1);
            return Ok(6);
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))
    }

    pub(in super::super) fn bld_abs(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let imm = Cpu::get_nibble_opcode(opcode2, 3)? & 7;
            let value = self.read_abs8_b(opcode as u8)?;
            self.write_ccr(CCR::C, (value >> imm) & 1);
            return Ok(6);
        };
        f().with_context(|| format!("opcode2 [{:x}]", opcode2))
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_bld_rn() {
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0x01).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x77, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // bit7
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0x80).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x77, 0x70]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        // register 0xf
        let mut cpu = Cpu::new();
        cpu.write_rn_b(0xf, 0x01).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x77, 0x0f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.write_rn_b(0, 0xfe).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x74, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr, 0);
    }

    #[test]
    fn test_bld_ern() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x01).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x77, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x80).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x77, 0x70]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // register 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0x01).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x70, 0x77, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffcf20, 0xfe).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7c, 0x00, 0x77, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }

    #[test]
    fn test_bld_abs() {
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0x01).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        // bit 7
        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0x80).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x70]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 1);

        let mut cpu = Cpu::new();
        cpu.write_abs24_b(0xffff12, 0xfe).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x7e, 0x12, 0x74, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr, 0);
    }
}
