use std::{future::Future, pin::Pin};

use crate::{
    cpu::{self},
    memory::MEMORY_START_ADDR,
};

pub struct TestHelper<ST, TT> {
    src_addressing_mode: Box<dyn AddressingMode<ST>>,
    target_addressing_mode: Box<dyn AddressingMode<TT>>,
}
impl<ST, TT> TestHelper<ST, TT>
where
    ST: Copy,
    TT: Copy,
{
    pub fn build(
        src_addressing_mode: Box<dyn AddressingMode<ST>>,
        target_addressing_mode: Box<dyn AddressingMode<TT>>,
    ) -> TestHelper<ST, TT> {
        TestHelper {
            src_addressing_mode,
            target_addressing_mode,
        }
    }

    // callback: (operator, src_index, target_index)
    pub async fn run<Fut>(&mut self, f: impl Fn(TestOperator, ST, TT) -> Fut)
    where
        Fut: Future,
    {
        let src_valid_index_list = self.src_addressing_mode.get_valid_index();
        let src_invalid_index_list = self.src_addressing_mode.get_invalid_index();
        let target_valid_index_list = self.target_addressing_mode.get_valid_index();
        let target_invalid_index_list = self.target_addressing_mode.get_invalid_index();

        for src_i in src_valid_index_list.iter() {
            for target_i in target_valid_index_list.iter() {
                f(TestOperator::new(true), *src_i, *target_i).await;
            }
            for target_i in target_invalid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i).await;
            }
        }
        for src_i in src_invalid_index_list.iter() {
            for target_i in target_valid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i).await;
            }
            for target_i in target_invalid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i).await;
            }
        }
    }
}

#[derive(Clone)]
pub struct TestOperator {
    cpu: cpu::Cpu,
    should_success: bool,
    initial_ccr: [u8; 2],
    expect_ccr: [u8; 2],
    expect_state: u8,
}

impl TestOperator {
    fn new(should_success: bool) -> TestOperator {
        TestOperator {
            cpu: cpu::Cpu::new(),
            should_success,
            initial_ccr: [0, 0xff], // Invariant values are 0 and 1
            expect_ccr: [0, 0xff],
            expect_state: 0,
        }
    }

    pub async fn exec<Fut>(self, f: impl Fn(cpu::Cpu) -> Fut)
    where
        Fut: Future<Output = bool>,
    {
        for i in 0..=1 {
            let mut cpu = self.cpu.clone();
            cpu.pc = MEMORY_START_ADDR;
            cpu.ccr = self.initial_ccr[i];
            let opcode = cpu.fetch().await;
            let result = cpu.exec(opcode).await;
            if self.should_success {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), self.expect_state);
                assert_eq!(cpu.ccr, self.expect_ccr[i]);
                assert!(f(cpu).await);
            } else {
                assert!(
                    result.is_err()
                        || result.is_ok_and(|state| state != self.expect_state)
                        || cpu.ccr != self.expect_ccr[1]
                        || !f(cpu).await
                );
            }
        }
    }

    pub async fn access_cpu<F>(mut self, f: F) -> TestOperator
    where
        F: for<'a> Fn(&'a mut cpu::Cpu) -> Pin<Box<dyn Future<Output = ()> + 'a>>,
    {
        f(&mut self.cpu).await;
        self
    }

    pub async fn set_opcode(mut self, opcode: &[u8]) -> TestOperator {
        self.cpu.bus.lock().await.memory[0..opcode.len()].copy_from_slice(opcode);
        self
    }

    pub fn should_success(mut self, success: bool) -> TestOperator {
        self.should_success = success;
        self
    }

    pub fn should_state(mut self, state: u8) -> TestOperator {
        self.expect_state = state;
        self
    }

    pub fn should_ccr_c(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1111_1110;
            self.expect_ccr[0] |= 0b0000_0001;
        } else {
            self.initial_ccr[0] |= 0b0000_0001;
            self.expect_ccr[1] &= 0b1111_1110;
        }
        self
    }

    pub fn should_ccr_v(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1111_1101;
            self.expect_ccr[0] |= 0b0000_0010;
        } else {
            self.initial_ccr[0] |= 0b0000_0010;
            self.expect_ccr[1] &= 0b1111_1101;
        }
        self
    }

    pub fn should_ccr_z(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1111_1011;
            self.expect_ccr[0] |= 0b0000_0100;
        } else {
            self.initial_ccr[0] |= 0b0000_0100;
            self.expect_ccr[1] &= 0b1111_1011;
        }
        self
    }

    pub fn should_ccr_n(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1111_0111;
            self.expect_ccr[0] |= 0b0000_1000;
        } else {
            self.initial_ccr[0] |= 0b0000_1000;
            self.expect_ccr[1] &= 0b1111_0111;
        }
        self
    }

    pub fn should_ccr_u(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1110_1111;
            self.expect_ccr[0] |= 0b0001_0000;
        } else {
            self.initial_ccr[0] |= 0b0001_0000;
            self.expect_ccr[1] &= 0b1110_1111;
        }
        self
    }

    pub fn should_ccr_h(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1101_1111;
            self.expect_ccr[0] |= 0b0010_0000;
        } else {
            self.initial_ccr[0] |= 0b0010_0000;
            self.expect_ccr[1] &= 0b1101_1111;
        }
        self
    }

    pub fn should_ccr_ui(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b1011_1111;
            self.expect_ccr[0] |= 0b0100_0000;
        } else {
            self.initial_ccr[0] |= 0b0100_0000;
            self.expect_ccr[1] &= 0b1011_1111;
        }
        self
    }

    pub fn should_ccr_i(mut self, on: bool) -> TestOperator {
        if on {
            self.initial_ccr[1] &= 0b0111_1111;
            self.expect_ccr[0] |= 0b1000_0000;
        } else {
            self.initial_ccr[0] |= 0b1000_0000;
            self.expect_ccr[1] &= 0b0111_1111;
        }
        self
    }
}

pub trait AddressingMode<T> {
    fn get_valid_index(&mut self) -> Vec<T>;
    fn get_invalid_index(&mut self) -> Vec<T>;
}

pub struct RnMode {
    valid_index_list: Vec<u8>,
    invalid_index_list: Vec<u8>,
}
impl RnMode {
    pub fn new() -> Box<RnMode> {
        Box::new(RnMode {
            valid_index_list: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],

            invalid_index_list: vec![],
        })
    }
}
impl AddressingMode<u8> for RnMode {
    fn get_valid_index(&mut self) -> Vec<u8> {
        self.valid_index_list.clone()
    }

    fn get_invalid_index(&mut self) -> Vec<u8> {
        self.invalid_index_list.clone()
    }
}

pub struct ImmMode {
    values: Vec<u8>,
}
impl ImmMode {
    pub fn new(value: u8) -> Box<ImmMode> {
        Box::new(ImmMode { values: [value].to_vec() })
    }
}
impl AddressingMode<u8> for ImmMode {
    fn get_valid_index(&mut self) -> Vec<u8> {
        self.values.clone()
    }

    fn get_invalid_index(&mut self) -> Vec<u8> {
        Vec::new()
    }
}

pub struct ErnMode {}
impl ErnMode {
    pub fn new() -> Box<ErnMode> {
        Box::new(ErnMode {})
    }
}
impl AddressingMode<u8> for ErnMode {
    fn get_valid_index(&mut self) -> Vec<u8> {
        vec![0, 1, 2, 3, 4, 5, 6, 7]
    }

    fn get_invalid_index(&mut self) -> Vec<u8> {
        Vec::new()
    }
}

pub struct Abs8Mode {}
impl Abs8Mode {
    pub fn new() -> Box<Abs8Mode> {
        Box::new(Abs8Mode {})
    }
}
impl AddressingMode<u8> for Abs8Mode {
    fn get_valid_index(&mut self) -> Vec<u8> {
        vec![0x02]
    }

    fn get_invalid_index(&mut self) -> Vec<u8> {
        Vec::new()
    }
}

pub struct Abs16Mode {}
impl Abs16Mode {
    pub fn new() -> Box<Abs16Mode> {
        Box::new(Abs16Mode {})
    }
}
impl AddressingMode<u16> for Abs16Mode {
    fn get_valid_index(&mut self) -> Vec<u16> {
        vec![0xff02]
    }

    fn get_invalid_index(&mut self) -> Vec<u16> {
        Vec::new()
    }
}

pub struct Abs24Mode {}
impl Abs24Mode {
    pub fn new() -> Box<Abs24Mode> {
        Box::new(Abs24Mode {})
    }
}
impl AddressingMode<u32> for Abs24Mode {
    fn get_valid_index(&mut self) -> Vec<u32> {
        vec![0x00ffff02]
    }

    fn get_invalid_index(&mut self) -> Vec<u32> {
        Vec::new()
    }
}
