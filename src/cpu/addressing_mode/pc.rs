use super::*;
use anyhow::{Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_pc8_b(&mut self, disp: u8, value: u8) -> Result<()> {
        if disp & 0x80 == 0x00 {
            self.write_abs24_b(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_b(self.pc + (0xffffff00 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc8_b(&self, disp: u8) -> Result<u8> {
        if disp & 0x80 == 0x00 {
            Ok(self
                .read_abs24_b(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_b(self.pc + (0xffffff00 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }

    pub(in super::super) fn write_pc8_w(&mut self, disp: u8, value: u16) -> Result<()> {
        if disp & 0x80 == 0x00 {
            self.write_abs24_w(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_w(self.pc + (0xffffff00 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc8_w(&self, disp: u8) -> Result<u16> {
        if disp & 0x80 == 0x00 {
            Ok(self
                .read_abs24_w(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_w(self.pc + (0xffffff00 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }

    pub(in super::super) fn write_pc8_l(&mut self, disp: u8, value: u32) -> Result<()> {
        if disp & 0x80 == 0x00 {
            self.write_abs24_l(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_l(self.pc + (0xffffff00 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc8_l(&self, disp: u8) -> Result<u32> {
        if disp & 0x80 == 0x00 {
            Ok(self
                .read_abs24_l(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_l(self.pc + (0xffffff00 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }

    pub(in super::super) fn write_pc16_b(&mut self, disp: u16, value: u8) -> Result<()> {
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_b(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_b(self.pc + (0xffff0000 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc16_b(&self, disp: u16) -> Result<u8> {
        if disp & 0x8000 == 0x0000 {
            Ok(self
                .read_abs24_b(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_b(self.pc + (0xffff0000 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }

    pub(in super::super) fn write_pc16_w(&mut self, disp: u16, value: u16) -> Result<()> {
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_w(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_w(self.pc + (0xffff0000 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc16_w(&self, disp: u16) -> Result<u16> {
        if disp & 0x8000 == 0x0000 {
            Ok(self
                .read_abs24_w(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_w(self.pc + (0xffff0000 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }

    pub(in super::super) fn write_pc16_l(&mut self, disp: u16, value: u32) -> Result<()> {
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_l(self.pc + disp as u32, value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        } else {
            self.write_abs24_l(self.pc + (0xffff0000 + disp as u32), value)
                .with_context(|| format!("disp [{:x}] value [{:x}]", disp, value))?;
        }
        Ok(())
    }

    pub(in super::super) fn read_pc16_l(&self, disp: u16) -> Result<u32> {
        if disp & 0x8000 == 0x0000 {
            Ok(self
                .read_abs24_l(self.pc + disp as u32)
                .with_context(|| format!("disp [{:x}]", disp))?)
        } else {
            Ok(self
                .read_abs24_l(self.pc + (0xffff0000 + disp as u32))
                .with_context(|| format!("disp [{:x}]", disp))?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

    #[test]
    fn test_write_pc8_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_pc8_b(0x10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);
    }

    #[test]
    fn test_read_pc8_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_pc8_b(0x10).unwrap(), 0xff);
    }

    #[test]
    fn test_write_pc8_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_pc8_w(0x10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_pc8_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_pc8_w(0x10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_pc8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_pc8_l(0x10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_pc8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xffff00;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_pc8_l(0x10).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_write_pc16_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_pc16_b(0x0f10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10).unwrap(), 0xff);
    }

    #[test]
    fn test_read_pc16_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff).unwrap();
        assert_eq!(cpu.read_pc16_b(0x0f10).unwrap(), 0xff);
    }

    #[test]
    fn test_write_pc16_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_pc16_w(0x0f10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_pc16_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_pc16_w(0x0f10).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_pc16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_pc16_l(0x0f10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_pc16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.pc = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_pc16_l(0x0f10).unwrap(), 0x0f0fff0f);
    }
}
