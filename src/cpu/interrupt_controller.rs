use anyhow::Result;
use std::collections::VecDeque;

use super::{Cpu, ADDRESS_MASK};

#[derive(Clone)]
pub(crate) struct InterruptController {
    interrupt_requests: VecDeque<u8>,
}

impl InterruptController {
    pub fn new() -> Self {
        InterruptController {
            interrupt_requests: VecDeque::new(),
        }
    }

    pub fn request_interrupt(&mut self, num: u8) {
        self.interrupt_requests.push_back(num);
    }
}

impl Cpu {
    pub(super) fn try_interrupt(&mut self) -> Result<()> {
        if let Some(vector) = self.interrupt_controller.interrupt_requests.pop_front() {
            self.interrupt(vector)?;
        }
        Ok(())
    }

    pub(super) fn interrupt(&mut self, vector: u8) -> Result<()> {
        self.write_dec_ern_l(7, ((self.ccr as u32) << 24) | self.pc)?;
        let vec_addr: u32 = (4 * vector).into();
        let dest_addr = self.read_abs24_l(vec_addr)?;
        self.pc = dest_addr & ADDRESS_MASK;
        self.write_ccr(crate::cpu::CCR::I, 1);
        Ok(())
    }
}
