use crate::cpu::Cpu;
use anyhow::{Context as _, Result};

impl<'a> Cpu<'a> {
    pub(in super::super) fn write_mem_indirect_b(&mut self, abs_addr: u8, value: u8) -> Result<()> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        self.write_abs24_b(addr & 0x00ffffff, value)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_mem_indirect_b(&self, abs_addr: u8) -> Result<u8> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))?;
        self.read_abs24_b(addr & 0x00ffffff)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }

    pub(in super::super) fn write_mem_indirect_w(
        &mut self,
        abs_addr: u8,
        value: u16,
    ) -> Result<()> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        self.write_abs24_w(addr & 0x00ffffff, value)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_mem_indirect_w(&self, abs_addr: u8) -> Result<u16> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))?;
        self.read_abs24_w(addr & 0x00ffffff)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }

    pub(in super::super) fn write_mem_indirect_l(
        &mut self,
        abs_addr: u8,
        value: u32,
    ) -> Result<()> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        self.write_abs24_l(addr & 0x00ffffff, value)
            .with_context(|| format!("abs_addr [{:x}] value [{:x}]", abs_addr, value))?;
        Ok(())
    }

    pub(in super::super) fn read_mem_indirect_l(&self, abs_addr: u8) -> Result<u32> {
        let addr = self
            .read_abs24_l(abs_addr as u32)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))?;
        self.read_abs24_l(addr & 0x00ffffff)
            .with_context(|| format!("abs_addr [{:x}]", abs_addr))
    }
}
