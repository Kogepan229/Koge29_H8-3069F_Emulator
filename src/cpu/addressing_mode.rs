use std::error::Error;

use super::*;

#[derive(Debug)]
struct AddressingError;

impl Error for AddressingError {}

impl std::fmt::Display for AddressingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Occurred addressing error.")
    }
}

impl<'a> Cpu<'a> {
    fn write_rn8(&mut self, register: Resister8, value: u8) -> Result<(), AddressingError> {
        match register as usize {
            // R0H..=R7H
            0..=7 => {
                self.er[register as usize] =
                    (self.er[register as usize] & 0xffff00ff) | ((value as u32) << 8)
            }
            // R0L..=R7L
            8..=15 => {
                self.er[register as usize - 8] =
                    (self.er[register as usize - 8] & 0xffffff00) | (value as u32)
            }
            _ => return Err(AddressingError),
        };
        Ok(())
    }

    fn read_Rn8(&self, register: Resister8) -> Result<u8, AddressingError> {
        match register as usize {
            // R0H..=R7H
            0..=7 => return Ok((self.er[register as usize] >> 8) as u8),
            // R0L..=R7L
            8..=15 => return Ok((self.er[register as usize - 8]) as u8),
            _ => return Err(AddressingError),
        };
    }

    fn write_Rn16(&self, register: Resister16, value: u16) -> Result<(), AddressingError> {
        Ok(())
    }

    fn read_Rn16(&self, register: Resister16) -> Result<u16, AddressingError> {
        Ok(0)
    }

    fn write_Rn32(&self, register: Resister32, value: u32) -> Result<(), AddressingError> {
        Ok(())
    }

    fn read_Rn32(&self, register: Resister32) -> Result<u32, AddressingError> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

    #[test]
    fn test_write_rn8() {
        let mcu = Mcu::new();
        let mut cpu = Cpu::new(&mcu);
        cpu.write_rn8(crate::cpu::Resister8::R0H, 0xff).unwrap();
        cpu.write_rn8(crate::cpu::Resister8::R7H, 0xff).unwrap();
        assert_eq!(cpu.er[0], 0x0000ff00);
        assert_eq!(cpu.er[7], 0x0000ff00);

        cpu.er[0] = 0;
        cpu.er[7] = 0;

        cpu.write_rn8(crate::cpu::Resister8::R0L, 0xff).unwrap();
        cpu.write_rn8(crate::cpu::Resister8::R7L, 0xff).unwrap();
        assert_eq!(cpu.er[0], 0x000000ff);
        assert_eq!(cpu.er[7], 0x000000ff);
    }

    #[test]
    fn test_read_rn8() {
        let mcu = Mcu::new();
        let mut cpu = Cpu::new(&mcu);
        cpu.er[0] = 0x0000ff00;
        cpu.er[7] = 0x0000ff00;
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R0H).unwrap(), 0xff);
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R1H).unwrap(), 0x00);
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R7H).unwrap(), 0xff);

        cpu.er[0] = 0x000000ff;
        cpu.er[7] = 0x000000ff;
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R0L).unwrap(), 0xff);
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R1L).unwrap(), 0x00);
        assert_eq!(cpu.read_Rn8(crate::cpu::Resister8::R7L).unwrap(), 0xff);
    }
}
