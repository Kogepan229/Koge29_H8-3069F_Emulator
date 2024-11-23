use crate::cpu::{Cpu, StateType};
use anyhow::Result;

impl Cpu {
    pub(in super::super) fn divxu_b(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)?;
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;

        let rd = self.read_rn_w(rd_i)?;
        let rs = self.read_rn_b(rs_i)?;

        if (rs as i8) < 0 {
            self.write_ccr(crate::cpu::CCR::N, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::N, 0);
        }

        if rs == 0 {
            self.write_ccr(crate::cpu::CCR::Z, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::Z, 0);
        }

        let quotient: u16 = if rs == 0 { 0 } else { rd / u16::from(rs) };
        let remainder: u16 = if rs == 0 { 0 } else { rd % u16::from(rs) };
        let result = (remainder << 8) | (quotient & 0xff);
        self.write_rn_w(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 12)?)
    }

    pub(in super::super) fn divxu_w(&mut self, opcode: u16) -> Result<u8> {
        let rd_i = Cpu::get_nibble_opcode(opcode, 4)? & 0b111;
        let rs_i = Cpu::get_nibble_opcode(opcode, 3)?;

        let rd = self.read_rn_l(rd_i)?;
        let rs = self.read_rn_w(rs_i)?;

        if (rs as i16) < 0 {
            self.write_ccr(crate::cpu::CCR::N, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::N, 0);
        }

        if rs == 0 {
            self.write_ccr(crate::cpu::CCR::Z, 1);
        } else {
            self.write_ccr(crate::cpu::CCR::Z, 0);
        }

        let quotient: u32 = if rs == 0 { 0 } else { rd / u32::from(rs) };
        let remainder: u32 = if rs == 0 { 0 } else { rd % u32::from(rs) };
        let result = (remainder << 16) | (quotient & 0xffff);
        self.write_rn_l(rd_i, result)?;

        Ok(self.calc_state(StateType::I, 1)? + self.calc_state(StateType::N, 20)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::testhelper::{ErnMode, RnMode, TestHelper};

    #[test]
    fn test_divxu_b() {
        TestHelper::build(RnMode::new(), RnMode::new()).run(|_operator, src_i, target_i| {
            let operator = _operator
                .set_opcode(&[0x51, (src_i << 4) | target_i])
                .should_state(14)
                .ignore(src_i % 8 == target_i % 8);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(target_i, 0).unwrap();
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    let result = cpu.read_rn_w(target_i).unwrap();
                    let quot = (result & 0xff) as u8;
                    let rem = (result >> 8) as u8;
                    assert_eq!(quot, 0);
                    assert_eq!(rem, 0);
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(target_i, 0xff).unwrap();
                    cpu.write_rn_b(src_i, 0).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    let result = cpu.read_rn_w(target_i).unwrap();
                    let quot = (result & 0xff) as u8;
                    let rem = (result >> 8) as u8;
                    assert_eq!(quot, 0);
                    assert_eq!(rem, 0);
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(target_i, 32767).unwrap(); // 0x7fff
                    cpu.write_rn_b(src_i, 255).unwrap(); // 0xff
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    let result = cpu.read_rn_w(target_i).unwrap();
                    let quot = (result & 0xff) as u8;
                    let rem = (result >> 8) as u8;
                    assert_eq!(quot, 128); // 0x80
                    assert_eq!(rem, 127); // 0x7f
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_w(target_i, -1i16 as u16).unwrap();
                    cpu.write_rn_b(src_i, 1).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(false)
                .exec(|cpu| {
                    let result = cpu.read_rn_w(target_i).unwrap();
                    let quot = (result & 0xff) as u8;
                    let rem = (result >> 8) as u8;
                    assert_eq!(quot, -1i8 as u8);
                    assert_eq!(rem, 0);
                    true
                });
        })
    }

    #[test]
    fn test_divxu_w() {
        TestHelper::build(RnMode::new(), ErnMode::new()).run(|_operator, src_i, target_i| {
            let operator = _operator
                .set_opcode(&[0x53, (src_i << 4) | target_i])
                .should_state(22)
                .ignore(src_i % 8 == target_i % 8);
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(target_i, 0).unwrap();
                    cpu.write_rn_w(src_i, 0).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    let result = cpu.read_rn_l(target_i).unwrap();
                    let quot = (result & 0xffff) as u16;
                    let rem = (result >> 16) as u16;
                    assert_eq!(quot, 0);
                    assert_eq!(rem, 0);
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(target_i, 0xffff).unwrap();
                    cpu.write_rn_w(src_i, 0).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(true)
                .exec(|cpu| {
                    let result = cpu.read_rn_l(target_i).unwrap();
                    let quot = (result & 0xffff) as u16;
                    let rem = (result >> 16) as u16;
                    assert_eq!(quot, 0);
                    assert_eq!(rem, 0);
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(target_i, 2_147_483_647).unwrap(); // 0x7fff_ffff
                    cpu.write_rn_w(src_i, 65_535).unwrap(); // 0xffff
                })
                .should_ccr_n(true)
                .should_ccr_z(false)
                .exec(|cpu| {
                    let result = cpu.read_rn_l(target_i).unwrap();
                    let quot = (result & 0xffff) as u16;
                    let rem = (result >> 16) as u16;
                    assert_eq!(quot, 32_768); // 0x8000
                    assert_eq!(rem, 32_767); // 0x7fff
                    true
                });
            operator
                .clone()
                .access_cpu(|cpu| {
                    cpu.write_rn_l(target_i, -1i32 as u32).unwrap();
                    cpu.write_rn_w(src_i, 1).unwrap();
                })
                .should_ccr_n(false)
                .should_ccr_z(false)
                .exec(|cpu| {
                    let result = cpu.read_rn_l(target_i).unwrap();
                    let quot = (result & 0xffff) as u16;
                    let rem = (result >> 16) as u16;
                    assert_eq!(quot, -1i16 as u16);
                    assert_eq!(rem, 0);
                    true
                });
        })
    }
}
