use std::error::Error;

use super::*;

#[derive(Debug, PartialEq)]
pub struct AddressingError;

impl Error for AddressingError {}

impl std::fmt::Display for AddressingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_name = write!(f, "Occurred addressing error.");
        var_name
    }
}

impl<'a> Cpu<'a> {
    pub(super) fn write_rn_b(
        &mut self,
        register_field: u8,
        value: u8,
    ) -> Result<(), AddressingError> {
        match register_field {
            // R0H..=R7H
            0..=7 => {
                self.er[register_field as usize] =
                    (self.er[register_field as usize] & 0xffff00ff) | ((value as u32) << 8)
            }
            // R0L..=R7L
            8..=15 => {
                self.er[register_field as usize - 8] =
                    (self.er[register_field as usize - 8] & 0xffffff00) | (value as u32)
            }
            _ => return Err(AddressingError),
        };
        Ok(())
    }

    pub(super) fn read_rn_b(&self, register_field: u8) -> Result<u8, AddressingError> {
        match register_field {
            // R0H..=R7H
            0..=7 => return Ok((self.er[register_field as usize] >> 8) as u8),
            // R0L..=R7L
            8..=15 => return Ok((self.er[register_field as usize - 8]) as u8),
            _ => return Err(AddressingError),
        };
    }

    pub(super) fn write_rn_w(
        &mut self,
        register_field: u8,
        value: u16,
    ) -> Result<(), AddressingError> {
        match register_field {
            // R0..=R7
            0..=7 => {
                self.er[register_field as usize] =
                    (self.er[register_field as usize] & 0xffff0000) | (value as u32)
            }
            // E0..=E7
            8..=15 => {
                self.er[register_field as usize - 8] =
                    (self.er[register_field as usize - 8] & 0x0000ffff) | ((value as u32) << 16)
            }
            _ => return Err(AddressingError),
        }
        Ok(())
    }

    pub(super) fn read_rn_w(&self, register_field: u8) -> Result<u16, AddressingError> {
        match register_field {
            // R0..=R7
            0..=7 => return Ok((self.er[register_field as usize]) as u16),
            // E0..=E7
            8..=15 => return Ok((self.er[register_field as usize - 8] >> 16) as u16),
            _ => return Err(AddressingError),
        };
    }

    pub(super) fn write_rn_l(
        &mut self,
        register_field: u8,
        value: u32,
    ) -> Result<(), AddressingError> {
        match register_field {
            // ER0..=ER7
            0..=7 => self.er[register_field as usize] = value,
            _ => return Err(AddressingError),
        }
        Ok(())
    }

    pub(super) fn read_rn_l(&self, register_field: u8) -> Result<u32, AddressingError> {
        match register_field {
            // ER0..=ER7
            0..=7 => return Ok(self.er[register_field as usize]),
            _ => return Err(AddressingError),
        }
    }

    pub(super) fn write_abs8_b(&mut self, addr: u8, value: u8) {
        self.mcu.write(0xffff00 | addr as u32, value).unwrap();
    }

    pub(super) fn read_abs8_b(&self, addr: u8) -> u8 {
        self.mcu.read(0xffff00 | addr as u32).unwrap()
    }

    pub(super) fn write_abs16_b(&mut self, addr: u16, value: u8) {
        if addr & 0x8000 == 0x0000 {
            self.mcu.write(addr as u32, value).unwrap();
        } else {
            self.mcu.write(0xff0000 | addr as u32, value).unwrap();
        }
    }

    pub(super) fn read_abs16_b(&self, addr: u16) -> u8 {
        if addr & 0x8000 == 0x0000 {
            return self.mcu.read(addr as u32).unwrap();
        } else {
            return self.mcu.read(0xff0000 | addr as u32).unwrap();
        }
    }

    pub(super) fn write_abs24_b(&mut self, addr: u32, value: u8) {
        self.mcu.write(addr, value).unwrap();
    }

    pub(super) fn read_abs24_b(&self, addr: u32) -> u8 {
        self.mcu.read(addr).unwrap()
    }

    pub(super) fn write_abs8_w(&mut self, addr: u8, value: u16) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        self.mcu
            .write(0xffff00 | addr as u32, (value >> 8) as u8)
            .unwrap();
        self.mcu
            .write((0xffff00 | addr as u32) + 1, value as u8)
            .unwrap();
        Ok(())
    }

    pub(super) fn read_abs8_w(&self, addr: u8) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        Ok((self.mcu.read(0xffff00 | addr as u32).unwrap() as u16) << 8
            | self.mcu.read((0xffff00 | addr as u32) + 1).unwrap() as u16)
    }

    pub(super) fn write_abs16_w(&mut self, addr: u16, value: u16) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        if addr & 0x8000 == 0x0000 {
            self.mcu.write(addr as u32, (value >> 8) as u8).unwrap();
            self.mcu.write((addr + 1) as u32, value as u8).unwrap();
        } else {
            self.mcu
                .write(0xff0000 | addr as u32, (value >> 8) as u8)
                .unwrap();
            self.mcu
                .write((0xff0000 | addr as u32) + 1, value as u8)
                .unwrap();
        }
        Ok(())
    }

    pub(super) fn read_abs16_w(&self, addr: u16) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        if addr & 0x8000 == 0x0000 {
            return Ok((self.mcu.read(addr as u32).unwrap() as u16) << 8
                | self.mcu.read((addr + 1) as u32).unwrap() as u16);
        } else {
            return Ok((self.mcu.read(0xff0000 | addr as u32).unwrap() as u16) << 8
                | self.mcu.read((0xff0000 | addr as u32) + 1).unwrap() as u16);
        }
    }

    pub(super) fn write_abs24_w(&mut self, addr: u32, value: u16) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        self.mcu.write(addr, (value >> 8) as u8).unwrap();
        self.mcu.write(addr + 1, value as u8).unwrap();
        Ok(())
    }

    pub(super) fn read_abs24_w(&self, addr: u32) -> Result<u16, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        Ok((self.mcu.read(addr).unwrap() as u16) << 8 | self.mcu.read(addr + 1).unwrap() as u16)
    }

    pub(super) fn write_abs8_l(&mut self, addr: u8, value: u32) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        self.write_abs24_w(0xffff00 | addr as u32, (value >> 16) as u16)
            .unwrap();
        self.write_abs24_w((0xffff00 | addr as u32) + 2, value as u16)
            .unwrap();
        Ok(())
    }

    pub(super) fn read_abs8_l(&self, addr: u8) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        Ok(
            (self.read_abs24_w(0xffff00 | addr as u32).unwrap() as u32) << 16
                | self.read_abs24_w((0xffff00 | addr as u32) + 2).unwrap() as u32,
        )
    }

    pub(super) fn write_abs16_l(&mut self, addr: u16, value: u32) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        if addr & 0x8000 == 0x0000 {
            self.write_abs24_w(addr as u32, (value >> 16) as u16)
                .unwrap();
            self.write_abs24_w((addr + 2) as u32, value as u16).unwrap();
        } else {
            self.write_abs24_w(0xff0000 | addr as u32, (value >> 16) as u16)
                .unwrap();
            self.write_abs24_w((0xff0000 | addr as u32) + 2, value as u16)
                .unwrap();
        }
        Ok(())
    }

    pub(super) fn read_abs16_l(&self, addr: u16) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        if addr & 0x8000 == 0x0000 {
            return Ok((self.read_abs24_w(addr as u32).unwrap() as u32) << 16
                | self.read_abs24_w((addr + 2) as u32).unwrap() as u32);
        } else {
            return Ok(
                (self.read_abs24_w(0xff0000 | addr as u32).unwrap() as u32) << 16
                    | self.read_abs24_w((0xff0000 | addr as u32) + 2).unwrap() as u32,
            );
        }
    }

    pub(super) fn write_abs24_l(&mut self, addr: u32, value: u32) -> Result<(), AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        self.write_abs24_w(addr, (value >> 16) as u16).unwrap();
        self.write_abs24_w(addr + 2, value as u16).unwrap();
        Ok(())
    }

    pub(super) fn read_abs24_l(&self, addr: u32) -> Result<u32, AddressingError> {
        if addr % 2 != 0 {
            return Err(AddressingError);
        }
        Ok((self.read_abs24_w(addr).unwrap() as u32) << 16
            | self.read_abs24_w(addr + 2).unwrap() as u32)
    }

    pub(super) fn write_ern_b(
        &mut self,
        register_field: u8,
        value: u8,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_b(addr & 0x00ffffff, value);
        Ok(())
    }

    pub(super) fn read_ern_b(&self, register_field: u8) -> Result<u8, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        Ok(self.read_abs24_b(addr & 0x00ffffff))
    }

    pub(super) fn write_ern_w(
        &mut self,
        register_field: u8,
        value: u16,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_w(addr & 0x00ffffff, value)?;
        Ok(())
    }

    pub(super) fn read_ern_w(&self, register_field: u8) -> Result<u16, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        self.read_abs24_w(addr & 0x00ffffff)
    }

    pub(super) fn write_ern_l(
        &mut self,
        register_field: u8,
        value: u32,
    ) -> Result<(), AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        self.write_abs24_l(addr & 0x00ffffff, value)?;
        Ok(())
    }

    pub(super) fn read_ern_l(&self, register_field: u8) -> Result<u32, AddressingError> {
        let addr = self.read_rn_l(register_field)?;
        self.read_abs24_l(addr & 0x00ffffff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{addressing_mode::AddressingError, Cpu},
        mcu::Mcu,
    };

    #[test]
    fn test_write_rn_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_rn_b(0b0000, 0xff).unwrap(); // R0H
        cpu.write_rn_b(0b0111, 0xff).unwrap(); // R7H
        assert_eq!(cpu.er[0], 0x0000ff00);
        assert_eq!(cpu.er[7], 0x0000ff00);

        cpu.er[0] = 0;
        cpu.er[7] = 0;

        cpu.write_rn_b(0b1000, 0xff).unwrap();
        cpu.write_rn_b(0b1111, 0xff).unwrap();
        assert_eq!(cpu.er[0], 0x000000ff);
        assert_eq!(cpu.er[7], 0x000000ff);
    }

    #[test]
    fn test_read_rn_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0x0000ff00;
        cpu.er[7] = 0x0000ff00;
        assert_eq!(cpu.read_rn_b(0b0000).unwrap(), 0xff); // R0H
        assert_eq!(cpu.read_rn_b(0b0001).unwrap(), 0x00); // R1H
        assert_eq!(cpu.read_rn_b(0b0111).unwrap(), 0xff); // R7H

        cpu.er[0] = 0x000000ff;
        cpu.er[7] = 0x000000ff;
        assert_eq!(cpu.read_rn_b(0b1000).unwrap(), 0xff); // R0L
        assert_eq!(cpu.read_rn_b(0b1001).unwrap(), 0x00); // R1L
        assert_eq!(cpu.read_rn_b(0b1111).unwrap(), 0xff); // R7L
    }

    #[test]
    fn test_write_rn_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_rn_w(0b0000, 0xffff).unwrap(); // R0
        cpu.write_rn_w(0b0111, 0xffff).unwrap(); // R7
        assert_eq!(cpu.er[0], 0x0000ffff);
        assert_eq!(cpu.er[7], 0x0000ffff);

        cpu.er[0] = 0;
        cpu.er[7] = 0;

        cpu.write_rn_w(0b1000, 0xffff).unwrap(); // E0
        cpu.write_rn_w(0b1111, 0xffff).unwrap(); // E7
        assert_eq!(cpu.er[0], 0xffff0000);
        assert_eq!(cpu.er[7], 0xffff0000);
    }

    #[test]
    fn test_read_rn_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0x0000ffff;
        cpu.er[7] = 0x0000ffff;
        assert_eq!(cpu.read_rn_w(0b0000).unwrap(), 0xffff); // R0
        assert_eq!(cpu.read_rn_w(0b0001).unwrap(), 0x0000); // R1
        assert_eq!(cpu.read_rn_w(0b0111).unwrap(), 0xffff); // R7

        cpu.er[0] = 0xffff0000;
        cpu.er[7] = 0xffff0000;
        assert_eq!(cpu.read_rn_w(0b1000).unwrap(), 0xffff); // E0
        assert_eq!(cpu.read_rn_w(0b1001).unwrap(), 0x0000); // E1
        assert_eq!(cpu.read_rn_w(0b1111).unwrap(), 0xffff); // E7
    }

    #[test]
    fn test_write_rn_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_rn_l(0b0000, 0xffffffff).unwrap(); // ER0
        cpu.write_rn_l(0b0001, 0xf0f0f0f0).unwrap(); // ER1
        cpu.write_rn_l(0b0111, 0x00000000).unwrap(); // ER7
        assert_eq!(cpu.er[0], 0xffffffff);
        assert_eq!(cpu.er[1], 0xf0f0f0f0);
        assert_eq!(cpu.er[7], 0x00000000);

        assert_eq!(
            cpu.write_rn_l(0b1000, 0xffffffff).unwrap_err(),
            AddressingError
        )
    }

    #[test]
    fn test_read_rn_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffffffff;
        cpu.er[7] = 0xf0f0f0f0;
        assert_eq!(cpu.read_rn_l(0b0000).unwrap(), 0xffffffff); // E0
        assert_eq!(cpu.read_rn_l(0b0001).unwrap(), 0x00000000); // E1
        assert_eq!(cpu.read_rn_l(0b0111).unwrap(), 0xf0f0f0f0); // E7
        assert_eq!(cpu.read_rn_l(0b1000).unwrap_err(), AddressingError); // E7
    }

    #[test]
    fn test_write_abs8_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_b(0x10, 0xff);
        cpu.write_abs8_b(0x1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs8_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_b(0x10), 0xff);
        assert_eq!(cpu.read_abs8_b(0x1f), 0xff);
    }

    #[test]
    fn test_write_abs16_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_b(0xff10, 0xff);
        cpu.write_abs16_b(0xff1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs16_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_b(0xff10), 0xff);
        assert_eq!(cpu.read_abs16_b(0xff1f), 0xff);
    }

    #[test]
    fn test_write_abs24_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_b(0xffff10, 0xff);
        cpu.write_abs24_b(0xffff1f, 0xff);
        assert_eq!(cpu.mcu.read(0xffff10).unwrap(), 0xff);
        assert_eq!(cpu.mcu.read(0xffff1f).unwrap(), 0xff);
    }

    #[test]
    fn test_read_abs24_b() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0xff).unwrap();
        cpu.mcu.write(0xffff1f, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_b(0xffff10), 0xff);
        assert_eq!(cpu.read_abs24_b(0xffff1f), 0xff);
    }

    #[test]
    fn test_write_abs8_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_w(0x10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );

        assert_eq!(cpu.write_abs8_w(0x11, 0xff).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_read_abs8_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs8_w(0x10).unwrap(), 0x0fff);

        assert_eq!(cpu.read_abs8_w(0x11).unwrap_err(), AddressingError)
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_w(0xff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );

        assert_eq!(
            cpu.write_abs16_w(0xff11, 0xf0f0).unwrap_err(),
            AddressingError
        )
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff00, 0x0f).unwrap();
        cpu.mcu.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff00).unwrap(), 0x0f0f);

        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs16_w(0xff10).unwrap(), 0x0fff);

        assert_eq!(cpu.read_abs16_w(0xff11).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_write_abs24_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(
            (cpu.mcu.read(0xffff10).unwrap() as u16) << 8 | cpu.mcu.read(0xffff11).unwrap() as u16,
            0x0fff
        );

        assert_eq!(
            cpu.write_abs24_w(0xffff11, 0xff).unwrap_err(),
            AddressingError
        )
    }

    #[test]
    fn test_read_abs24_w() {
        let mut mcu = Mcu::new();
        let cpu = Cpu::new(&mut mcu);
        cpu.mcu.write(0xffff00, 0x0f).unwrap();
        cpu.mcu.write(0xffff01, 0x0f).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff00).unwrap(), 0x0f0f);

        cpu.mcu.write(0xffff10, 0x0f).unwrap();
        cpu.mcu.write(0xffff11, 0xff).unwrap();
        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff);

        assert_eq!(cpu.read_abs24_w(0xffff11).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_write_abs8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs8_l(0x10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );

        assert_eq!(cpu.write_abs8_l(0x11, 0xff).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_read_abs8_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff10, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff12, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs8_l(0x10).unwrap(), 0x0f0fff0f);

        assert_eq!(cpu.read_abs8_l(0x11).unwrap_err(), AddressingError)
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_write_abs16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs16_l(0xff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );

        assert_eq!(
            cpu.write_abs16_l(0xff11, 0xf0f0).unwrap_err(),
            AddressingError
        )
    }

    // アドレスの最上位ビットが1のときのみ
    #[test]
    fn test_read_abs16_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs16_l(0xff00).unwrap(), 0x0f0fff0f);

        assert_eq!(cpu.read_abs16_l(0xff11).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_write_abs24_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(
            (cpu.read_abs24_w(0xffff10).unwrap() as u32) << 16
                | cpu.read_abs24_w(0xffff12).unwrap() as u32,
            0x0f0fff0f
        );

        assert_eq!(
            cpu.write_abs24_l(0xffff11, 0xff).unwrap_err(),
            AddressingError
        )
    }

    #[test]
    fn test_read_abs24_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.write_abs24_w(0xffff00, 0x0f0f).unwrap();
        cpu.write_abs24_w(0xffff02, 0xff0f).unwrap();
        assert_eq!(cpu.read_abs24_l(0xffff00).unwrap(), 0x0f0fff0f);

        assert_eq!(cpu.read_abs24_l(0xffff11).unwrap_err(), AddressingError)
    }

    #[test]
    fn test_write_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_b(0, 0xff).unwrap();

        assert_eq!(cpu.read_abs24_b(0xffff10), 0xff)
    }

    #[test]
    fn test_read_ern_b() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_b(0xffff10, 0xff);
        assert_eq!(cpu.read_ern_b(0).unwrap(), 0xff)
    }

    #[test]
    fn test_write_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_w(0, 0x0fff).unwrap();

        assert_eq!(cpu.read_abs24_w(0xffff10).unwrap(), 0x0fff)
    }

    #[test]
    fn test_read_ern_w() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_w(0xffff10, 0x0fff).unwrap();
        assert_eq!(cpu.read_ern_w(0).unwrap(), 0x0fff)
    }

    #[test]
    fn test_write_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_ern_l(0, 0x0f0fff0f).unwrap();

        assert_eq!(cpu.read_abs24_l(0xffff10).unwrap(), 0x0f0fff0f)
    }

    #[test]
    fn test_read_ern_l() {
        let mut mcu = Mcu::new();
        let mut cpu = Cpu::new(&mut mcu);
        cpu.er[0] = 0xffff10;
        cpu.write_abs24_l(0xffff10, 0x0f0fff0f).unwrap();
        assert_eq!(cpu.read_ern_l(0).unwrap(), 0x0f0fff0f)
    }
}
