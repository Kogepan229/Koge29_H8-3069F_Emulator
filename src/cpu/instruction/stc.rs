use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn stc_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        self.write_rn_b(rd_i, self.ccr)?;

        self.calc_state(StateType::I, 1)
    }

    pub(in super::super) fn stc_w_ern(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        let addr = self.read_rn_l(erd_i)?;
        self.write_ern_w(erd_i, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::M, 1, addr)?)
    }

    pub(in super::super) fn stc_w_disp16(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        let disp = self.fetch();
        let addr = self.get_addr_disp16(erd_i, disp)?;
        self.write_disp16_ern_w(erd_i, disp, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 3)? + self.calc_state_with_addr(StateType::M, 1, addr)?)
    }

    pub(in super::super) fn stc_w_disp24(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)?;
        self.fetch(); // opcode3
        let opcode4 = self.fetch();
        let opcode5 = self.fetch();
        let disp = (u32::from(opcode4) << 16) | u32::from(opcode5);
        let addr = self.get_addr_disp24(erd_i, disp)?;
        self.write_disp24_ern_w(erd_i, disp, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 5)? + self.calc_state_with_addr(StateType::M, 1, addr)?)
    }

    pub(in super::super) fn stc_w_inc_ern(&mut self, opcode2: u16) -> Result<u8> {
        let erd_i = Cpu::get_nibble_opcode(opcode2, 3)? & 0b111;
        let addr = self.read_rn_l(erd_i)?;
        self.write_inc_ern_w(erd_i, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 2)? + self.calc_state_with_addr(StateType::M, 1, addr)? + self.calc_state(StateType::N, 2)?)
    }

    pub(in super::super) fn stc_abs16(&mut self) -> Result<u8> {
        let addr = self.fetch();
        let read_addr = self.get_addr_abs16(addr);
        self.write_abs16_w(addr, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 3)? + self.calc_state_with_addr(StateType::M, 1, read_addr)?)
    }

    pub(in super::super) fn stc_abs24(&mut self) -> Result<u8> {
        let opcode3 = self.fetch();
        let opcode4 = self.fetch();
        let addr = (u32::from(opcode3) << 16) | u32::from(opcode4);
        self.write_abs24_w(addr, u16::from(self.ccr))?;

        Ok(self.calc_state(StateType::I, 4)? + self.calc_state_with_addr(StateType::M, 1, addr)?)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::cpu::testhelper::{Abs16Mode, Abs24Mode, Disp16Mode, Disp24Mode, ErnMode, ImmMode, IncErnMode, RnMode, TestHelper};

    #[test]
    fn test_stc_b() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), RnMode::new()).run(|operator, ccr, target_i| {
            operator
                .clone()
                .set_opcode(&[0x02, target_i])
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                })
                .should_check_ccr(false)
                .should_state(2)
                .exec(|cpu| {
                    assert_eq!(cpu.read_rn_b(target_i).unwrap(), ccr);
                    true
                });
        });
    }

    #[test]
    fn test_stc_w_ern() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), ErnMode::new()).run(|operator, ccr, target_i| {
            operator
                .clone()
                .set_opcode(&[0x01, 0x40, 0x69, (target_i << 4) | 0x80])
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                })
                .should_state(6)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_ern_w(target_i).unwrap(), ccr.into());
                    true
                });
        });
    }

    #[test]
    fn test_stc_w_disp16() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), Disp16Mode::new()).run(|operator, ccr, disp| {
            operator
                .clone()
                .set_opcode(&[0x01, 0x40, 0x6f, (disp.er_i << 4) | 0x80, disp.as8(1), disp.as8(2)])
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                    cpu.write_rn_l(disp.er_i, disp.base_addr).unwrap();
                })
                .should_state(8)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_disp16_ern_w(disp.er_i, disp.disp).unwrap(), ccr.into());
                    true
                });
        });
    }

    #[test]
    fn test_stc_w_disp24() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), Disp24Mode::new()).run(|operator, ccr, disp| {
            operator
                .clone()
                .set_opcode(&[
                    0x01,
                    0x40,
                    0x78,
                    (disp.er_i << 4),
                    0x6b,
                    0xa0,
                    disp.as8(1),
                    disp.as8(2),
                    disp.as8(3),
                    disp.as8(4),
                ])
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                    cpu.write_rn_l(disp.er_i, disp.base_addr).unwrap();
                })
                .should_state(12)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_disp24_ern_w(disp.er_i, disp.disp).unwrap(), ccr.into());
                    true
                });
        });
    }

    #[test]
    fn test_stc_w_inc_ern() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), IncErnMode::new_w()).run(|operator, ccr, inc| {
            operator
                .clone()
                .set_opcode(&[0x01, 0x40, 0x6d, (inc.er_i << 4) | 0x80])
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                    cpu.write_rn_l(inc.er_i, inc.base_addr).unwrap()
                })
                .should_state(8)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_abs24_w(inc.base_addr).unwrap(), ccr.into());
                    assert_eq!(cpu.read_rn_l(inc.er_i).unwrap(), inc.result_addr);
                    true
                });
        })
    }

    #[test]
    fn test_stc_w_abs16() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), Abs16Mode::new()).run(|operator, ccr, abs| {
            operator
                .clone()
                .set_opcode(&[&[0x01, 0x40, 0x6b, 0x80], &abs.to_be_bytes()[..]].concat())
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                })
                .should_state(8)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_abs16_w(abs).unwrap(), ccr.into());
                    true
                });
        })
    }

    #[test]
    fn test_stc_w_abs24() {
        // Using ImmMode as ccr
        TestHelper::build(ImmMode::new(vec![0, 0xf]), Abs24Mode::new()).run(|operator, ccr, abs| {
            operator
                .clone()
                .set_opcode(&[&[0x01, 0x40, 0x6b, 0xa0], &abs.to_be_bytes()[..]].concat())
                .access_cpu(|cpu| {
                    cpu.ccr = ccr;
                })
                .should_state(10)
                .should_check_ccr(false)
                .exec(|cpu| {
                    assert_eq!(cpu.read_abs24_w(abs).unwrap(), ccr.into());
                    true
                });
        })
    }
}
