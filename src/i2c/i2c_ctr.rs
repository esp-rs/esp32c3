#[doc = "Reader of register I2C_CTR"]
pub type R = crate::R<u32, super::I2C_CTR>;
#[doc = "Writer for register I2C_CTR"]
pub type W = crate::W<u32, super::I2C_CTR>;
#[doc = "Register I2C_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_ADDR_BROADCASTING_EN`"]
pub type I2C_ADDR_BROADCASTING_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ADDR_BROADCASTING_EN`"]
pub struct I2C_ADDR_BROADCASTING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ADDR_BROADCASTING_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `I2C_ADDR_10BIT_RW_CHECK_EN`"]
pub type I2C_ADDR_10BIT_RW_CHECK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ADDR_10BIT_RW_CHECK_EN`"]
pub struct I2C_ADDR_10BIT_RW_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ADDR_10BIT_RW_CHECK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2C_SLV_TX_AUTO_START_EN`"]
pub type I2C_SLV_TX_AUTO_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SLV_TX_AUTO_START_EN`"]
pub struct I2C_SLV_TX_AUTO_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SLV_TX_AUTO_START_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `I2C_CONF_UPGATE`"]
pub struct I2C_CONF_UPGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CONF_UPGATE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `I2C_FSM_RST`"]
pub struct I2C_FSM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_FSM_RST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2C_ARBITRATION_EN`"]
pub type I2C_ARBITRATION_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_ARBITRATION_EN`"]
pub struct I2C_ARBITRATION_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_ARBITRATION_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `I2C_CLK_EN`"]
pub type I2C_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_CLK_EN`"]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2C_RX_LSB_FIRST`"]
pub type I2C_RX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RX_LSB_FIRST`"]
pub struct I2C_RX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RX_LSB_FIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `I2C_TX_LSB_FIRST`"]
pub type I2C_TX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_TX_LSB_FIRST`"]
pub struct I2C_TX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TX_LSB_FIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `I2C_TRANS_START`"]
pub struct I2C_TRANS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_TRANS_START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2C_MS_MODE`"]
pub type I2C_MS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_MS_MODE`"]
pub struct I2C_MS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_MS_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2C_RX_FULL_ACK_LEVEL`"]
pub type I2C_RX_FULL_ACK_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_RX_FULL_ACK_LEVEL`"]
pub struct I2C_RX_FULL_ACK_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_RX_FULL_ACK_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `I2C_SAMPLE_SCL_LEVEL`"]
pub type I2C_SAMPLE_SCL_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SAMPLE_SCL_LEVEL`"]
pub struct I2C_SAMPLE_SCL_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SAMPLE_SCL_LEVEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2C_SCL_FORCE_OUT`"]
pub type I2C_SCL_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SCL_FORCE_OUT`"]
pub struct I2C_SCL_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_FORCE_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2C_SDA_FORCE_OUT`"]
pub type I2C_SDA_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C_SDA_FORCE_OUT`"]
pub struct I2C_SDA_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SDA_FORCE_OUT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c_addr_broadcasting_en(&self) -> I2C_ADDR_BROADCASTING_EN_R {
        I2C_ADDR_BROADCASTING_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_addr_10bit_rw_check_en(&self) -> I2C_ADDR_10BIT_RW_CHECK_EN_R {
        I2C_ADDR_10BIT_RW_CHECK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_slv_tx_auto_start_en(&self) -> I2C_SLV_TX_AUTO_START_EN_R {
        I2C_SLV_TX_AUTO_START_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_arbitration_en(&self) -> I2C_ARBITRATION_EN_R {
        I2C_ARBITRATION_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&self) -> I2C_RX_LSB_FIRST_R {
        I2C_RX_LSB_FIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&self) -> I2C_TX_LSB_FIRST_R {
        I2C_TX_LSB_FIRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_ms_mode(&self) -> I2C_MS_MODE_R {
        I2C_MS_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_rx_full_ack_level(&self) -> I2C_RX_FULL_ACK_LEVEL_R {
        I2C_RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&self) -> I2C_SAMPLE_SCL_LEVEL_R {
        I2C_SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_scl_force_out(&self) -> I2C_SCL_FORCE_OUT_R {
        I2C_SCL_FORCE_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_sda_force_out(&self) -> I2C_SDA_FORCE_OUT_R {
        I2C_SDA_FORCE_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c_addr_broadcasting_en(&mut self) -> I2C_ADDR_BROADCASTING_EN_W {
        I2C_ADDR_BROADCASTING_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c_addr_10bit_rw_check_en(&mut self) -> I2C_ADDR_10BIT_RW_CHECK_EN_W {
        I2C_ADDR_10BIT_RW_CHECK_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c_slv_tx_auto_start_en(&mut self) -> I2C_SLV_TX_AUTO_START_EN_W {
        I2C_SLV_TX_AUTO_START_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c_conf_upgate(&mut self) -> I2C_CONF_UPGATE_W {
        I2C_CONF_UPGATE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c_fsm_rst(&mut self) -> I2C_FSM_RST_W {
        I2C_FSM_RST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2c_arbitration_en(&mut self) -> I2C_ARBITRATION_EN_W {
        I2C_ARBITRATION_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&mut self) -> I2C_RX_LSB_FIRST_W {
        I2C_RX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&mut self) -> I2C_TX_LSB_FIRST_W {
        I2C_TX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_trans_start(&mut self) -> I2C_TRANS_START_W {
        I2C_TRANS_START_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_ms_mode(&mut self) -> I2C_MS_MODE_W {
        I2C_MS_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_rx_full_ack_level(&mut self) -> I2C_RX_FULL_ACK_LEVEL_W {
        I2C_RX_FULL_ACK_LEVEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&mut self) -> I2C_SAMPLE_SCL_LEVEL_W {
        I2C_SAMPLE_SCL_LEVEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_scl_force_out(&mut self) -> I2C_SCL_FORCE_OUT_W {
        I2C_SCL_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_sda_force_out(&mut self) -> I2C_SDA_FORCE_OUT_W {
        I2C_SDA_FORCE_OUT_W { w: self }
    }
}
