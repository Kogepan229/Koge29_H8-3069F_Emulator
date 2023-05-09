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
    fn write_rn8(&mut self, register_field: u8, value: u8) -> Result<(), AddressingError> {
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

    fn read_rn8(&self, register_field: u8) -> Result<u8, AddressingError> {
        match register_field as usize {
            // R0H..=R7H
            0..=7 => return Ok((self.er[register_field as usize] >> 8) as u8),
            // R0L..=R7L
            8..=15 => return Ok((self.er[register_field as usize - 8]) as u8),
            _ => return Err(AddressingError),
        };
    }

    fn write_rn16(&self, register_field: u8, value: u16) -> Result<(), AddressingError> {
        Ok(())
    }

    fn read_rn16(&self, register_field: u8) -> Result<u16, AddressingError> {
        Ok(0)
    }

    fn write_rn32(&self, register_field: u8, value: u32) -> Result<(), AddressingError> {
        Ok(())
    }

    fn read_rn32(&self, register_field: u8) -> Result<u32, AddressingError> {
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
        cpu.write_rn8(0b0000, 0xff).unwrap(); // R0H
        cpu.write_rn8(0b0111, 0xff).unwrap(); // R7H
        assert_eq!(cpu.er[0], 0x0000ff00);
        assert_eq!(cpu.er[7], 0x0000ff00);

        cpu.er[0] = 0;
        cpu.er[7] = 0;

        cpu.write_rn8(0b1000, 0xff).unwrap();
        cpu.write_rn8(0b1111, 0xff).unwrap();
        assert_eq!(cpu.er[0], 0x000000ff);
        assert_eq!(cpu.er[7], 0x000000ff);
    }

    #[test]
    fn test_read_rn8() {
        let mcu = Mcu::new();
        let mut cpu = Cpu::new(&mcu);
        cpu.er[0] = 0x0000ff00;
        cpu.er[7] = 0x0000ff00;
        assert_eq!(cpu.read_rn8(0b0000).unwrap(), 0xff); // R0H
        assert_eq!(cpu.read_rn8(0b0001).unwrap(), 0x00); // R1H
        assert_eq!(cpu.read_rn8(0b0111).unwrap(), 0xff); // R7H

        cpu.er[0] = 0x000000ff;
        cpu.er[7] = 0x000000ff;
        assert_eq!(cpu.read_rn8(0b1000).unwrap(), 0xff); // R0L
        assert_eq!(cpu.read_rn8(0b1001).unwrap(), 0x00); // R1L
        assert_eq!(cpu.read_rn8(0b1111).unwrap(), 0xff); // R7L
    }
}
