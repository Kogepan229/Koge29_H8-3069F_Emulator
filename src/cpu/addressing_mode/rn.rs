use anyhow::{bail, Result};

use super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_rn_b(&mut self, register_field: u8, value: u8) -> Result<()> {
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
            _ => bail!("Invalid register [{:x}]", register_field),
        };
        Ok(())
    }

    pub(in super::super) fn read_rn_b(&self, register_field: u8) -> Result<u8> {
        match register_field {
            // R0H..=R7H
            0..=7 => return Ok((self.er[register_field as usize] >> 8) as u8),
            // R0L..=R7L
            8..=15 => return Ok((self.er[register_field as usize - 8]) as u8),
            _ => bail!("Invalid register [{:x}]", register_field),
        };
    }

    pub(in super::super) fn write_rn_w(&mut self, register_field: u8, value: u16) -> Result<()> {
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
            _ => bail!("Invalid register [{:x}]", register_field),
        }
        Ok(())
    }

    pub(in super::super) fn read_rn_w(&self, register_field: u8) -> Result<u16> {
        match register_field {
            // R0..=R7
            0..=7 => return Ok((self.er[register_field as usize]) as u16),
            // E0..=E7
            8..=15 => return Ok((self.er[register_field as usize - 8] >> 16) as u16),
            _ => bail!("Invalid register [{:x}]", register_field),
        };
    }

    pub(in super::super) fn write_rn_l(&mut self, register_field: u8, value: u32) -> Result<()> {
        match register_field {
            // ER0..=ER7
            0..=7 => self.er[register_field as usize] = value,
            _ => bail!("Invalid register [{:x}]", register_field),
        }
        Ok(())
    }

    pub(in super::super) fn read_rn_l(&self, register_field: u8) -> Result<u32> {
        match register_field {
            // ER0..=ER7
            0..=7 => return Ok(self.er[register_field as usize]),
            _ => bail!("Invalid register [{:x}]", register_field),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::Cpu, mcu::Mcu};

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

        assert!(cpu.write_rn_l(0b1000, 0xffffffff).is_err())
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
        assert!(cpu.read_rn_l(0b1000).is_err()); // E7
    }
}
