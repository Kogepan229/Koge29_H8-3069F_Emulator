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
    pub fn run(&mut self, f: impl Fn(TestOperator, ST, TT)) {
        let src_valid_index_list = self.src_addressing_mode.get_valid_index();
        let src_invalid_index_list = self.src_addressing_mode.get_invalid_index();
        let target_valid_index_list = self.target_addressing_mode.get_valid_index();
        let target_invalid_index_list = self.target_addressing_mode.get_invalid_index();

        for src_i in src_valid_index_list.iter() {
            for target_i in target_valid_index_list.iter() {
                f(TestOperator::new(true), *src_i, *target_i);
            }
            for target_i in target_invalid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i);
            }
        }
        for src_i in src_invalid_index_list.iter() {
            for target_i in target_valid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i);
            }
            for target_i in target_invalid_index_list.iter() {
                f(TestOperator::new(false), *src_i, *target_i);
            }
        }
    }
}

#[derive(Clone)]
pub struct TestOperator {
    cpu: cpu::Cpu,
    should_success: bool,
    should_check_ccr: bool,
    initial_ccr: [u8; 2],
    expect_ccr: [u8; 2],
    expect_state: u8,
}

impl TestOperator {
    fn new(should_success: bool) -> TestOperator {
        TestOperator {
            cpu: cpu::Cpu::new(),
            should_success,
            should_check_ccr: true,
            initial_ccr: [0, 0xff], // Invariant values are 0 and 1
            expect_ccr: [0, 0xff],
            expect_state: 0,
        }
    }

    pub fn exec(self, f: impl Fn(cpu::Cpu) -> bool) {
        for i in 0..=1 {
            let mut cpu = self.cpu.clone();
            cpu.pc = MEMORY_START_ADDR;
            if self.should_check_ccr {
                cpu.ccr = self.initial_ccr[i];
            }
            let opcode = cpu.fetch();
            let result = cpu.exec(opcode);
            if self.should_success {
                assert_eq!(result.unwrap(), self.expect_state);
                if self.should_check_ccr {
                    assert_eq!(cpu.ccr, self.expect_ccr[i]);
                }
                assert!(f(cpu));
            } else {
                let mut is_err = false;
                if self.should_check_ccr {
                    is_err = is_err || cpu.ccr != self.expect_ccr[1];
                }
                is_err = is_err || result.is_err() || result.is_ok_and(|state| state != self.expect_state) || !f(cpu);
                assert!(is_err);
            }
        }
    }

    pub fn access_cpu(mut self, f: impl Fn(&mut cpu::Cpu)) -> TestOperator {
        f(&mut self.cpu);
        self
    }

    pub fn set_opcode(mut self, opcode: &[u8]) -> TestOperator {
        self.cpu.bus.memory[0..opcode.len()].copy_from_slice(opcode);
        self
    }

    pub fn should_success(mut self, success: bool) -> TestOperator {
        self.should_success = success;
        self
    }

    pub fn should_check_ccr(mut self, check: bool) -> TestOperator {
        self.should_check_ccr = check;
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

pub struct NoneMode {}
impl NoneMode {
    pub fn new() -> Box<NoneMode> {
        Box::new(NoneMode {})
    }
}
impl AddressingMode<()> for NoneMode {
    fn get_valid_index(&mut self) -> Vec<()> {
        vec![()]
    }

    fn get_invalid_index(&mut self) -> Vec<()> {
        vec![]
    }
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
    pub fn new(values: Vec<u8>) -> Box<ImmMode> {
        Box::new(ImmMode { values })
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

#[derive(Clone, Copy)]
pub struct Disp16Data {
    pub disp: u16,
    pub base_addr: u32,
    pub target_addr: u32,
    pub er_i: u8,
}
impl Disp16Data {
    pub fn as8(&self, index: u8) -> u8 {
        match index {
            1 => return (self.disp >> 8) as u8,
            2 => return self.disp as u8,
            _ => panic!(),
        }
    }
}

pub struct Disp16Mode {}
impl Disp16Mode {
    pub fn new() -> Box<Disp16Mode> {
        Box::new(Disp16Mode {})
    }
}
impl AddressingMode<Disp16Data> for Disp16Mode {
    fn get_valid_index(&mut self) -> Vec<Disp16Data> {
        let list: Vec<Disp16Data> = vec![0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .map(|i| Disp16Data {
                disp: 0x0eee,
                base_addr: 0xffcf20,
                target_addr: 0xffde0e,
                er_i: *i,
            })
            .collect();
        list
    }

    fn get_invalid_index(&mut self) -> Vec<Disp16Data> {
        Vec::new()
    }
}

#[derive(Clone, Copy)]
pub struct Disp24Data {
    pub disp: u32,
    pub base_addr: u32,
    pub target_addr: u32,
    pub er_i: u8,
}
impl Disp24Data {
    pub fn as8(&self, index: u8) -> u8 {
        match index {
            1 => return (self.disp >> 24) as u8,
            2 => return (self.disp >> 16) as u8,
            3 => return (self.disp >> 8) as u8,
            4 => return self.disp as u8,
            _ => panic!(),
        }
    }
}

pub struct Disp24Mode {}
impl Disp24Mode {
    pub fn new() -> Box<Disp24Mode> {
        Box::new(Disp24Mode {})
    }
}
impl AddressingMode<Disp24Data> for Disp24Mode {
    fn get_valid_index(&mut self) -> Vec<Disp24Data> {
        let list: Vec<Disp24Data> = vec![0, 1, 2, 3, 4, 5, 6, 7]
            .iter()
            .map(|i| Disp24Data {
                disp: 0xfffeee,
                base_addr: 0xffcf20,
                target_addr: 0xffce0e,
                er_i: *i,
            })
            .collect();
        list
    }

    fn get_invalid_index(&mut self) -> Vec<Disp24Data> {
        Vec::new()
    }
}

#[derive(Clone, Copy)]
pub struct IncErnData {
    pub base_addr: u32,
    pub result_addr: u32,
    pub er_i: u8,
}

pub struct IncErnMode {
    valid_index_list: Vec<IncErnData>,
}
impl IncErnMode {
    fn new(diff: u32) -> Box<IncErnMode> {
        Box::new(IncErnMode {
            valid_index_list: vec![0, 1, 2, 3, 4, 5, 6, 7]
                .iter()
                .map(|i| IncErnData {
                    base_addr: 0xffcf20,
                    result_addr: 0xffcf20 + diff,
                    er_i: *i,
                })
                .collect(),
        })
    }

    pub fn new_b() -> Box<IncErnMode> {
        IncErnMode::new(1)
    }

    pub fn new_w() -> Box<IncErnMode> {
        IncErnMode::new(2)
    }

    pub fn new_l() -> Box<IncErnMode> {
        IncErnMode::new(4)
    }
}
impl AddressingMode<IncErnData> for IncErnMode {
    fn get_valid_index(&mut self) -> Vec<IncErnData> {
        self.valid_index_list.clone()
    }

    fn get_invalid_index(&mut self) -> Vec<IncErnData> {
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
