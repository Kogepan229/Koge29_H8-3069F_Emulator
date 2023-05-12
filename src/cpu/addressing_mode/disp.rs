use super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_disp16_ern_b(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u8,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_b((addr + disp as u32) & 0xffffff, value);
        } else {
            self.write_abs24_b((addr + (0xffff0000 + disp as u32)) & 0xffffff, value);
        }
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_b(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u8, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            Ok(self.read_abs24_b((addr + disp as u32) & 0xffffff))
        } else {
            Ok(self.read_abs24_b((addr + (0xffff0000 + disp as u32)) & 0xffffff))
        }
    }

    pub(in super::super) fn write_disp16_ern_w(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u16,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_w((addr + disp as u32) & 0xffffff, value)?;
        } else {
            self.write_abs24_w((addr + (0xffff0000 + disp as u32)) & 0xffffff, value)?;
        }
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_w(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u16, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            Ok(self.read_abs24_w((addr + disp as u32) & 0xffffff)?)
        } else {
            Ok(self.read_abs24_w((addr + (0xffff0000 + disp as u32)) & 0xffffff)?)
        }
    }

    pub(in super::super) fn write_disp16_ern_l(
        &mut self,
        register_field: u8,
        disp: u16,
        value: u32,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            self.write_abs24_l((addr + disp as u32) & 0xffffff, value)?;
        } else {
            self.write_abs24_l((addr + (0xffff0000 + disp as u32)) & 0xffffff, value)?;
        }
        Ok(())
    }

    pub(in super::super) fn read_disp16_ern_l(
        &self,
        register_field: u8,
        disp: u16,
    ) -> Result<u32, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x8000 == 0x0000 {
            Ok(self.read_abs24_l((addr + disp as u32) & 0xffffff)?)
        } else {
            Ok(self.read_abs24_l((addr + (0xffff0000 + disp as u32)) & 0xffffff)?)
        }
    }

    ////
    pub(in super::super) fn write_disp24_ern_b(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u8,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            self.write_abs24_b((addr + disp) & 0xffffff, value);
        } else {
            self.write_abs24_b((addr + (0xff000000 + disp)) & 0xffffff, value);
        }
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_b(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u8, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            Ok(self.read_abs24_b((addr + disp) & 0xffffff))
        } else {
            Ok(self.read_abs24_b((addr + (0xff000000 + disp)) & 0xffffff))
        }
    }

    pub(in super::super) fn write_disp24_ern_w(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u16,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            self.write_abs24_w((addr + disp) & 0xffffff, value)?;
        } else {
            self.write_abs24_w((addr + (0xff000000 + disp)) & 0xffffff, value)?;
        }
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_w(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u16, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            Ok(self.read_abs24_w((addr + disp) & 0xffffff)?)
        } else {
            Ok(self.read_abs24_w((addr + (0xff000000 + disp)) & 0xffffff)?)
        }
    }

    pub(in super::super) fn write_disp24_ern_l(
        &mut self,
        register_field: u8,
        disp: u32,
        value: u32,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            self.write_abs24_l((addr + disp) & 0xffffff, value)?;
        } else {
            self.write_abs24_l((addr + (0xff000000 + disp)) & 0xffffff, value)?;
        }
        Ok(())
    }

    pub(in super::super) fn read_disp24_ern_l(
        &self,
        register_field: u8,
        disp: u32,
    ) -> Result<u32, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        if disp & 0x800000 == 0x000000 {
            Ok(self.read_abs24_l((addr + disp) & 0xffffff)?)
        } else {
            Ok(self.read_abs24_l((addr + (0xff000000 + disp)) & 0xffffff)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

    #[test]
    fn test_write_disp16_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_b(0, 0x0f10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10), 0xff);

        cpu.er[1] = 0x000110;
        cpu.write_disp16_ern_b(1, 0xf002, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xfff112), 0xff);
    }

    #[test]
    fn test_read_disp16_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff);
        assert_eq!(cpu.read_disp16_ern_b(0, 0x0f10).unwrap(), 0xff);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_b(0xfff112, 0xff);
        assert_eq!(cpu.read_disp16_ern_b(1, 0xf002).unwrap(), 0xff);
    }

    #[test]
    fn test_write_disp16_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_w(0, 0x0f10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);

        cpu.er[1] = 0x000110;
        cpu.write_disp16_ern_w(1, 0xf002, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xfff112).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_disp16_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp16_ern_w(0, 0x0f10).unwrap(), 0x0fff);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_w(0xfff112, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp16_ern_w(1, 0xf002).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_disp16_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp16_ern_l(0, 0x0f10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0x000110;
        cpu.write_disp16_ern_l(1, 0xf002, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xfff112).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_disp16_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp16_ern_l(0, 0x0f10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_l(0xfff112, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp16_ern_l(1, 0xf002).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_write_disp24_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_b(0, 0x000f10, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10), 0xff);

        cpu.er[1] = 0x000110;
        cpu.write_disp24_ern_b(1, 0xfff002, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xfff112), 0xff);
    }

    #[test]
    fn test_read_disp24_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_b(0xffff10, 0xff);
        assert_eq!(cpu.read_disp24_ern_b(0, 0x000f10).unwrap(), 0xff);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_b(0xfff112, 0xff);
        assert_eq!(cpu.read_disp24_ern_b(1, 0xfff002).unwrap(), 0xff);
    }

    #[test]
    fn test_write_disp24_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_w(0, 0x000f10, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);

        cpu.er[1] = 0x000110;
        cpu.write_disp24_ern_w(1, 0xfff002, 0x0fff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xfff112).unwrap(), 0x0fff);
    }

    #[test]
    fn test_read_disp24_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp24_ern_w(0, 0x000f10).unwrap(), 0x0fff);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_w(0xfff112, 0x0fff).unwrap();
        assert_eq!(cpu.read_disp24_ern_w(1, 0xfff002).unwrap(), 0x0fff);
    }

    #[test]
    fn test_write_disp24_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_disp24_ern_l(0, 0x000f10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0x000110;
        cpu.write_disp24_ern_l(1, 0xfff002, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xfff112).unwrap(), 0x0f0fff0f);
    }

    #[test]
    fn test_read_disp24_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xfff000;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp24_ern_l(0, 0x000f10).unwrap(), 0x0f0fff0f);

        cpu.er[1] = 0x000110;
        cpu.write_abs24_l(0xfff112, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_disp24_ern_l(1, 0xfff002).unwrap(), 0x0f0fff0f);
    }
}
