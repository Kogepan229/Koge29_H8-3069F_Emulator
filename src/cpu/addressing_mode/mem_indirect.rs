use super::*;

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_mem_indirect_b(&mut self, abs_addr: u8, value: u8) {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.write_abs24_b(addr & 0x00ffffff, value);
    }

    pub(in super::super) fn read_mem_indirect_b(&self, abs_addr: u8) -> u8 {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.read_abs24_b(addr & 0x00ffffff)
    }

    pub(in super::super) fn write_mem_indirect_w(
        &mut self,
        abs_addr: u8,
        value: u16,
    ) -> Result<(), AddressingError> {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.write_abs24_w(addr & 0x00ffffff, value)?;
        Ok(())
    }

    pub(in super::super) fn read_mem_indirect_w(
        &self,
        abs_addr: u8,
    ) -> Result<u16, AddressingError> {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.read_abs24_w(addr & 0x00ffffff)
    }

    pub(in super::super) fn write_mem_indirect_l(
        &mut self,
        abs_addr: u8,
        value: u32,
    ) -> Result<(), AddressingError> {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.write_abs24_l(addr & 0x00ffffff, value)?;
        Ok(())
    }

    pub(in super::super) fn read_mem_indirect_l(
        &self,
        abs_addr: u8,
    ) -> Result<u32, AddressingError> {
        let addr = self.read_abs24_l(abs_addr as u32).unwrap();
        self.read_abs24_l(addr & 0x00ffffff)
    }
}
