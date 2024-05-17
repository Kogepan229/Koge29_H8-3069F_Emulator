use crate::cpu::{Cpu, StateType, CCR};
use anyhow::{bail, Context as _, Result};

impl Cpu {
    pub(in super::super) async fn mov_b(&mut self, opcode: u16) -> Result<u8> {
        match (opcode >> 8) as u8 {
            0x0c => return self.mov_b_rn(opcode).await,
            0xf0..=0xff => return self.mov_b_imm(opcode).await,
            0x68 => return self.mov_b_ern(opcode).await,
            0x6e => return self.mov_b_disp16(opcode).await,
            0x6c => return self.mov_b_inc_or_dec(opcode).await,
            0x20..=0x2f | 0x30..=0x3f => return self.mov_b_abs8(opcode).await,
            0x6a => return self.mov_b_abs_16_or_24(opcode).await,
            _ => bail!("invalid opcode [{:>04x}]", opcode),
        }
    }

    fn mov_b_proc_pcc(&mut self, src: u8) {
        if (src as i8) < 0 {
            self.write_ccr(CCR::N, 1);
        } else {
            self.write_ccr(CCR::N, 0);
        }
        if src == 0 {
            self.write_ccr(CCR::Z, 1);
        } else {
            self.write_ccr(CCR::Z, 0);
        }
        self.write_ccr(CCR::V, 0);
    }

    async fn mov_b_rn(&mut self, opcode: u16) -> Result<u8> {
        let mut f = || -> Result<()> {
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 3)?)?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
            return Ok(());
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    async fn mov_b_imm(&mut self, opcode: u16) -> Result<u8> {
        let mut f = || -> Result<()> {
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 2)?, opcode as u8)?;
            self.mov_b_proc_pcc(opcode as u8);
            return Ok(());
        };
        f().with_context(|| format!("opcode [{:x}]", opcode))?;
        Ok(self.calc_state(StateType::I, 1).await?)
    }

    async fn mov_b_ern(&mut self, opcode: u16) -> Result<u8> {
        if opcode & 0x0080 == 0 {
            let register_ern = Cpu::get_nibble_opcode(opcode, 3)?;
            let access_addr = self.get_addr_ern(register_ern)?;
            let value = self.read_ern_b(register_ern).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 1).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        } else {
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
            let register_ern = Cpu::get_nibble_opcode(opcode, 3)? & 0x07;
            let access_addr = self.get_addr_ern(register_ern)?;
            self.write_ern_b(register_ern, value).await?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 1).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        }
    }

    async fn mov_b_disp16(&mut self, opcode: u16) -> Result<u8> {
        let disp = self.fetch().await;
        if opcode & 0x0080 == 0 {
            let register_disp_ern = Cpu::get_nibble_opcode(opcode, 3)?;
            let access_addr = self.get_addr_disp16(register_disp_ern, disp)?;
            let value = self.read_disp16_ern_b(register_disp_ern, disp).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 2).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        } else {
            let register_disp_ern = Cpu::get_nibble_opcode(opcode, 3)? & 0x07;
            let access_addr = self.get_addr_disp16(register_disp_ern, disp)?;
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
            self.write_disp16_ern_b(register_disp_ern, disp, value).await?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 2).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        }
    }

    pub(in super::super) async fn mov_b_disp24(&mut self, opcode: u16, opcode2: u16) -> Result<u8> {
        let disp = ((self.fetch().await as u32) << 16) | self.fetch().await as u32;
        if opcode2 & 0xfff0 == 0x6a20 {
            let register_disp_ern = Cpu::get_nibble_opcode(opcode, 3)?;
            let access_addr = self.get_addr_disp24(register_disp_ern, disp)?;
            let value = self.read_disp24_ern_b(register_disp_ern, disp).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode2, 4)?, value)?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 4).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        } else {
            let register_disp_ern = Cpu::get_nibble_opcode(opcode, 3)? & 0x07;
            let access_addr = self.get_addr_disp24(register_disp_ern, disp)?;
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode2, 4)?)?;
            self.write_disp24_ern_b(register_disp_ern, disp, value).await?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 4).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
        }
    }

    async fn mov_b_inc_or_dec(&mut self, opcode: u16) -> Result<u8> {
        if opcode & 0x0080 == 0 {
            let register_ern = Cpu::get_nibble_opcode(opcode, 3)?;
            let access_addr = self.read_rn_l(register_ern)? & 0x00ffffff;
            let value = self.read_inc_ern_b(register_ern).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 1).await?
                + self.calc_state_with_addr(StateType::L, 1, access_addr).await?
                + self.calc_state(StateType::N, 2).await?)
        } else {
            let register_ern = Cpu::get_nibble_opcode(opcode, 3)? & 0x07;
            let access_addr = (self.read_rn_l(register_ern)? - 1) & 0x00ffffff;
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
            self.write_dec_ern_b(register_ern, value).await?;
            self.mov_b_proc_pcc(value);
            Ok(self.calc_state(StateType::I, 1).await?
                + self.calc_state_with_addr(StateType::L, 1, access_addr).await?
                + self.calc_state(StateType::N, 2).await?)
        }
    }

    async fn mov_b_abs8(&mut self, opcode: u16) -> Result<u8> {
        let access_addr = self.get_addr_abs8(opcode as u8);
        if opcode & 0xf000 == 0x2000 {
            let value = self.read_abs8_b(opcode as u8).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 2)?, value)?;
            self.mov_b_proc_pcc(value);
        } else {
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 2)?)?;
            self.write_abs8_b(opcode as u8, value).await?;
            self.mov_b_proc_pcc(value);
        }
        Ok(self.calc_state(StateType::I, 1).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
    }

    async fn mov_b_abs_16_or_24(&mut self, opcode: u16) -> Result<u8> {
        match opcode & 0xfff0 {
            0x6a00 | 0x6a80 => return self.mov_b_abs16(opcode).await,
            0x6a20 | 0x6aa0 => return self.mov_b_abs24(opcode).await,
            _ => bail!("invalid opcode [{:x}]", opcode),
        }
    }

    async fn mov_b_abs16(&mut self, opcode: u16) -> Result<u8> {
        let abs_addr = self.fetch().await;
        let access_addr = self.get_addr_abs16(abs_addr);
        if opcode & 0xfff0 == 0x6a00 {
            let value = self.read_abs16_b(abs_addr).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
        } else {
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
            self.write_abs16_b(abs_addr, value).await?;
            self.mov_b_proc_pcc(value);
        }
        Ok(self.calc_state(StateType::I, 2).await? + self.calc_state_with_addr(StateType::L, 1, access_addr).await?)
    }

    async fn mov_b_abs24(&mut self, opcode: u16) -> Result<u8> {
        let abs_addr = ((self.fetch().await as u32) << 16) | self.fetch().await as u32;
        if opcode & 0xfff0 == 0x6a20 {
            let value = self.read_abs24_b(abs_addr).await?;
            self.write_rn_b(Cpu::get_nibble_opcode(opcode, 4)?, value)?;
            self.mov_b_proc_pcc(value);
        } else {
            let value = self.read_rn_b(Cpu::get_nibble_opcode(opcode, 4)?)?;
            self.write_abs24_b(abs_addr, value).await?;
            self.mov_b_proc_pcc(value);
        }
        Ok(self.calc_state(StateType::I, 3).await? + self.calc_state_with_addr(StateType::L, 1, abs_addr).await?)
    }
}

#[cfg(test)]
mod tests {
    use nom::AsBytes;

    use crate::{
        cpu::{
            testhelper::{Abs16Mode, Abs24Mode, Abs8Mode, ErnMode, ImmMode, RnMode, TestHelper},
            Cpu,
        },
        memory::MEMORY_START_ADDR,
    };

    #[tokio::test]
    async fn test_mov_b_rn_helper() {
        TestHelper::build(RnMode::new(), RnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x0c, (src_i << 4) | target_i])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                        })
                    })
                    .await
                    .should_state(2)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x0c, (src_i << 4) | target_i])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0).unwrap();
                        })
                    })
                    .await
                    .should_state(2)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_imm_helper() {
        TestHelper::build(ImmMode::new(0xa5), RnMode::new())
            .run(|operator, imm, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0xf0 | target_i, imm])
                    .await
                    .should_state(2)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
            })
            .await;
        TestHelper::build(ImmMode::new(0x00), RnMode::new())
            .run(|operator, imm, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0xf0 | target_i, imm])
                    .await
                    .should_state(2)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_ern_to_rn_helper() {
        TestHelper::build(ErnMode::new(), RnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x68, (src_i << 4) | target_i])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffcf20, 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x68, (src_i << 4) | target_i])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0).unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_ern_helper() {
        TestHelper::build(RnMode::new(), ErnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x68, (target_i << 4) | src_i | 0x80])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffcf20).await.unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x68, (target_i << 4) | src_i | 0x80])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffcf20).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_disp16_to_rn_helper() {
        TestHelper::build(ErnMode::new(), RnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x6e, (src_i << 4) | target_i, 0x0e, 0xee])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffde0e, 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x6e, (src_i << 4) | target_i, 0x0e, 0xee])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffde0e, 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_disp16_helper() {
        TestHelper::build(RnMode::new(), ErnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x6e, (target_i << 4) | src_i | 0x80, 0x0e, 0xee])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffde0e).await.unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x6e, (target_i << 4) | src_i | 0x80, 0x0e, 0xee])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffde0e).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_disp24_to_rn_helper() {
        TestHelper::build(ErnMode::new(), RnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x78, (src_i << 4), 0x6a, 0x20 | target_i, 0x00, 0xff, 0xfe, 0xee])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffce0e, 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(10)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x78, (src_i << 4), 0x6a, 0x20 | target_i, 0x00, 0xff, 0xfe, 0xee])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffce0e, 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(10)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_disp24_helper() {
        TestHelper::build(RnMode::new(), ErnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x78, (target_i << 4), 0x6a, 0xa0 | src_i, 0x00, 0xff, 0xfe, 0xee])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(10)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffce0e).await.unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x78, (target_i << 4), 0x6a, 0xa0 | src_i, 0x00, 0xff, 0xfe, 0xee])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf20).unwrap();
                        })
                    })
                    .await
                    .should_state(10)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffce0e).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_inc_helper() {
        TestHelper::build(ErnMode::new(), RnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x6c, (src_i << 4) | target_i])
                    .await
                    .should_success(src_i != target_i % 8) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffcf20, 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 && cpu.read_rn_l(src_i).unwrap() == 0xffcf21 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x6c, (src_i << 4) | target_i])
                    .await
                    .should_success(src_i != target_i % 8) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_l(src_i, 0xffcf20).unwrap();
                            cpu.write_abs24_b(0xffcf20, 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 && cpu.read_rn_l(src_i).unwrap() == 0xffcf21 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_dec_helper() {
        TestHelper::build(RnMode::new(), ErnMode::new())
            .run(|operator, src_i, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x6c, (target_i << 4) | src_i | 0x80])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf21).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffcf20).await.unwrap() == 0xa5 && cpu.read_rn_l(target_i).unwrap() == 0xffcf20 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x6c, (target_i << 4) | src_i | 0x80])
                    .await
                    .should_success(src_i % 8 != target_i) // Avoid conflict
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                            cpu.write_rn_l(target_i, 0xffcf21).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffcf20).await.unwrap() == 0x00 && cpu.read_rn_l(target_i).unwrap() == 0xffcf20 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_abs8_to_rn_helper() {
        TestHelper::build(Abs8Mode::new(), RnMode::new())
            .run(|operator, abs, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x20 | target_i, abs])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b(0xffff00 | (abs as u32), 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x20 | target_i, abs])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b(0xffff00 | (abs as u32), 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_abs8_helper() {
        TestHelper::build(RnMode::new(), Abs8Mode::new())
            .run(|operator, src_i, abs| async move {
                operator // negative value
                    .clone()
                    .set_opcode(&[0x30 | src_i, abs])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffff00 | (abs as u32)).await.unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode(&[0x30 | src_i, abs])
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                        })
                    })
                    .await
                    .should_state(4)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xffff00 | (abs as u32)).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_abs16_to_rn_helper() {
        TestHelper::build(Abs16Mode::new(), RnMode::new())
            .run(|operator, abs, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, target_i], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b(0xff0000 | (abs as u32), 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode([&[0x6a, target_i], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b(0xff0000 | (abs as u32), 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_abs16_helper() {
        TestHelper::build(RnMode::new(), Abs16Mode::new())
            .run(|operator, src_i, abs| async move {
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, src_i | 0x80], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xff0000 | (abs as u32)).await.unwrap() == 0xa5 })
                    .await;
                operator // zero value
                    .clone()
                    .set_opcode([&[0x6a, src_i | 0x80], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                        })
                    })
                    .await
                    .should_state(6)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b(0xff0000 | (abs as u32)).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_abs24_to_rn_helper() {
        TestHelper::build(Abs24Mode::new(), RnMode::new())
            .run(|operator, abs, target_i| async move {
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, target_i | 0x20], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b((abs << 4) >> 4, 0xa5).await.unwrap();
                        })
                    })
                    .await
                    .should_state(8)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0xa5 })
                    .await;
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, target_i | 0x20], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_abs24_b((abs << 4) >> 4, 0x00).await.unwrap();
                        })
                    })
                    .await
                    .should_state(8)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_rn_b(target_i).unwrap() == 0x00 })
                    .await;
            })
            .await;
    }

    #[tokio::test]
    async fn test_mov_b_rn_to_abs24_helper() {
        TestHelper::build(RnMode::new(), Abs24Mode::new())
            .run(|operator, src_i, abs| async move {
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, src_i | 0xa0], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0xa5).unwrap();
                        })
                    })
                    .await
                    .should_state(8)
                    .should_ccr_v(false)
                    .should_ccr_z(false)
                    .should_ccr_n(true)
                    .exec(|cpu| async move { cpu.read_abs24_b((abs << 4) >> 4).await.unwrap() == 0xa5 })
                    .await;
                operator // negative value
                    .clone()
                    .set_opcode([&[0x6a, src_i | 0xa0], abs.to_be_bytes().as_bytes()].concat().as_bytes())
                    .await
                    .access_cpu(|cpu| {
                        Box::pin(async move {
                            cpu.write_rn_b(src_i, 0x00).unwrap();
                        })
                    })
                    .await
                    .should_state(8)
                    .should_ccr_v(false)
                    .should_ccr_z(true)
                    .should_ccr_n(false)
                    .exec(|cpu| async move { cpu.read_abs24_b((abs << 4) >> 4).await.unwrap() == 0x00 })
                    .await;
            })
            .await;
    }
}
