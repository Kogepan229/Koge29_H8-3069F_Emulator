use crate::cpu::{Cpu, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super) fn mov_w(&mut self, opcode: u16) -> Result<usize> {
        match (opcode >> 8) as u8 {
            0x0d => return self.mov_w_rn(opcode),
            0x79 => return self.mov_w_imm(opcode),
            0x69 => return self.mov_w_ern(opcode),
            0x6f => return self.mov_w_disp16(opcode),
            0x6d => return self.mov_w_inc_or_dec(opcode),
            0x6b => return self.mov_w_abs(opcode),
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn mov_w_proc_pcc(&mut self, src: u16) {
        if (src as i16) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if src == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
    }

    fn mov_w_rn(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
            self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_w_proc_pcc(value);
            return Ok(2);
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))
    }

    fn mov_w_imm(&mut self, opcode: u16) -> Result<usize> {
        let imm = self.fetch();
        let mut f = || -> Result<usize> {
            self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, imm)?;
            self.mov_w_proc_pcc(imm);
            return Ok(4);
        };
        f().with_context(|| format!("opcode [{:x}] imm(opcode2) [{:x}]", opcode, imm))
    }

    fn mov_w_ern(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_ern_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(4);
        };
        f()
    }

    fn mov_w_disp16(&mut self, opcode: u16) -> Result<usize> {
        let disp = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_disp16_ern_w(Cpu::get_nibble_opcode(opcode, 3)?, disp)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_disp16_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, disp, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f().with_context(|| format!("disp [{:x}]", disp))
    }

    pub(in super::super) fn mov_w_disp24(&mut self, opcode: u16, opcode2: u16) -> Result<usize> {
        let disp = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode2 & 0xfff0 == 0x6b20 {
                let value = self.read_disp24_ern_w(Cpu::get_nibble_opcode(opcode, 3)?, disp)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode2, 4)?)?;
                self.write_disp24_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, disp, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(10);
        };
        f().with_context(|| format!("opcode2 [{:x}] disp [{:x}]", opcode2, disp))
    }

    fn mov_w_inc_or_dec(&mut self, opcode: u16) -> Result<usize> {
        let mut f = || -> Result<usize> {
            if opcode & 0x0080 == 0 {
                let value = self.read_inc_ern_w(Cpu::get_nibble_opcode(opcode, 3)?)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_dec_ern_w(Cpu::get_nibble_opcode(opcode, 3)? & 0x07, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f()
    }

    fn mov_w_abs(&mut self, opcode: u16) -> Result<usize> {
        match opcode & 0xfff0 {
            0x6b00 | 0x6b80 => return self.mov_w_abs16(opcode),
            0x6b20 | 0x6ba0 => return self.mov_w_abs24(opcode),
            _ => bail!("invalid opcode [{:x}]", opcode),
        }
    }

    fn mov_w_abs16(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = self.fetch();
        let mut f = || -> Result<usize> {
            if opcode & 0xfff0 == 0x6b00 {
                let value = self.read_abs16_w(abs_addr)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_abs16_w(abs_addr, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(6);
        };
        f().with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }

    fn mov_w_abs24(&mut self, opcode: u16) -> Result<usize> {
        let abs_addr = ((self.fetch() as u32) << 16) | self.fetch() as u32;
        let mut f = || -> Result<usize> {
            if opcode & 0xfff0 == 0x6b20 {
                let value = self.read_abs24_w(abs_addr)?;
                self.write_rn_w(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
                self.mov_w_proc_pcc(value);
            } else {
                let value = self.read_rn_w(Cpu::get_nibble_opcode(opcode, 4)?)?;
                self.write_abs24_w(abs_addr, value)?;
                self.mov_w_proc_pcc(value);
            }
            return Ok(8);
        };
        f().with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    #[test]
    fn test_mov_w_rn() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.bus.memory[0..2].copy_from_slice(&[0x0d, 0x0f]);
        cpu.write_rn_w(0, 0xb6a5).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.bus.memory[0..2].copy_from_slice(&[0x0d, 0xf0]);
        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.bus.memory[0..2].copy_from_slice(&[0x0d, 0x0f]);
        cpu.write_rn_w(0, 0).unwrap();
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 2);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_imm() {
        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.bus.memory[0..4].copy_from_slice(&[0x79, 0x00, 0xb6, 0xa5]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.bus.memory[0..4].copy_from_slice(&[0x79, 0x0f, 0xb6, 0xa5]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.bus.memory[0..4].copy_from_slice(&[0x79, 0x00, 0x00, 0x00]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_ern() {
        ////////
        // EAs to Rd

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0xb6a5).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0x0f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0xb6a5).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0x70]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0x0f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);

        ////////
        // Rs to ERs

        cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0x8f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0xb6a5);

        cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0, 0).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x69, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 4);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_disp16() {
        ////////
        // EAs to Rd

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffde0e, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0x0f, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffde0e, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0x70, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffde0e, 0).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0x0f, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);

        ////////
        // Rs to ERs

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0x8f, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffde0e).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0xf0, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffde0e).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0xf, 0).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6f, 0x8f, 0x0e, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffde0e).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_disp24() {
        ////////
        // EAs to Rd

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffce0e, 0xb6a5).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x00, 0x6b, 0x2f, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffce0e, 0xb6a5).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x70, 0x6b, 0x20, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffce0e, 0).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x00, 0x6b, 0x2f, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);

        ////////
        // Rs to ERs

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x00, 0x6b, 0xaf, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffce0e).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x70, 0x6b, 0xa0, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffce0e).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0xf, 0).unwrap();
        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.bus.memory[0..8].copy_from_slice(&[0x78, 0x00, 0x6b, 0xaf, 0x00, 0xff, 0xfe, 0xee]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 10);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffce0e).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_inc_or_dec() {
        ////////
        // increment

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0xb6a5).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0x0f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xffcf22);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_l(7, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0xb6a5).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0x70]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0xffcf22);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_l(0, 0xffcf20).unwrap();
        cpu.write_abs24_w(0xffcf20, 0).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0x0f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xffcf22);

        ////////
        // decrement

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.write_rn_l(0, 0xffcf22).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0x8f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0xb6a5);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xffcf20);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.write_rn_l(7, 0xffcf22).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0xf0]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0xb6a5);
        assert_eq!(cpu.read_rn_l(7).unwrap(), 0xffcf20);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0xf, 0).unwrap();
        cpu.write_rn_l(0, 0xffcf22).unwrap();
        cpu.bus.memory[0..2].copy_from_slice(&[0x6d, 0x8f]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffcf20).unwrap(), 0);
        assert_eq!(cpu.read_rn_l(0).unwrap(), 0xffcf20);
    }

    #[test]
    fn test_mov_w_abs16() {
        ////////
        // EAs to Rd

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_abs24_w(0xffff02, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x00, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_abs24_w(0xffff02, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x0f, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_abs24_w(0xffff02, 0).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x00, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);

        ////////
        // Rs to ERs

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x80, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x8f, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0, 0).unwrap();
        cpu.bus.memory[0..4].copy_from_slice(&[0x6b, 0x80, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 6);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0);
    }

    #[test]
    fn test_mov_w_abs24() {
        ////////
        // EAs to Rd

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_abs24_w(0xffff02, 0xb6a5).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0x20, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_abs24_w(0xffff02, 0xb6a5).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0x2f, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_rn_w(0xf).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_abs24_w(0xffff02, 0).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0x20, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_rn_w(0).unwrap(), 0);

        ////////
        // Rs to ERs

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0, 0xb6a5).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0xa0, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x04;

        cpu.write_rn_w(0xf, 0xb6a5).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0xaf, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00001000);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0xb6a5);

        let mut cpu = Cpu::new();
        cpu.ccr = 0x0a;

        cpu.write_rn_w(0, 0).unwrap();
        cpu.bus.memory[0..6].copy_from_slice(&[0x6b, 0xa0, 0x00, 0xff, 0xff, 0x02]);
        let opcode = cpu.fetch();
        let state = cpu.exec(opcode).unwrap();
        assert_eq!(state, 8);
        assert_eq!(cpu.ccr & 0b00001110, 0b00000100);
        assert_eq!(cpu.read_abs24_w(0xffff02).unwrap(), 0);
    }
}
