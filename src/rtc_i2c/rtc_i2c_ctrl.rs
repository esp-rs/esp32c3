#[doc = "Reader of register RTC_I2C_CTRL"]
pub type R = crate::R<u32, super::RTC_I2C_CTRL>;
#[doc = "Writer for register RTC_I2C_CTRL"]
pub type W = crate::W<u32, super::RTC_I2C_CTRL>;
#[doc = "Register RTC_I2C_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_CLK_EN`"]
pub type RTC_I2C_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_CLK_EN`"]
pub struct RTC_I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_RESET`"]
pub type RTC_I2C_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_RESET`"]
pub struct RTC_I2C_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_CTRL_CLK_GATE_EN`"]
pub type RTC_I2C_CTRL_CLK_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_CTRL_CLK_GATE_EN`"]
pub struct RTC_I2C_CTRL_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_CTRL_CLK_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RTC_I2C_RX_LSB_FIRST`"]
pub type RTC_I2C_RX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_RX_LSB_FIRST`"]
pub struct RTC_I2C_RX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_RX_LSB_FIRST_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TX_LSB_FIRST`"]
pub type RTC_I2C_TX_LSB_FIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TX_LSB_FIRST`"]
pub struct RTC_I2C_TX_LSB_FIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TX_LSB_FIRST_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_TRANS_START`"]
pub type RTC_I2C_TRANS_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_TRANS_START`"]
pub struct RTC_I2C_TRANS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_TRANS_START_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_MS_MODE`"]
pub type RTC_I2C_MS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_MS_MODE`"]
pub struct RTC_I2C_MS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_MS_MODE_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SCL_FORCE_OUT`"]
pub type RTC_I2C_SCL_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SCL_FORCE_OUT`"]
pub struct RTC_I2C_SCL_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SCL_FORCE_OUT_W<'a> {
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
#[doc = "Reader of field `RTC_I2C_SDA_FORCE_OUT`"]
pub type RTC_I2C_SDA_FORCE_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_I2C_SDA_FORCE_OUT`"]
pub struct RTC_I2C_SDA_FORCE_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_SDA_FORCE_OUT_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_i2c_clk_en(&self) -> RTC_I2C_CLK_EN_R {
        RTC_I2C_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_i2c_reset(&self) -> RTC_I2C_RESET_R {
        RTC_I2C_RESET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_i2c_ctrl_clk_gate_en(&self) -> RTC_I2C_CTRL_CLK_GATE_EN_R {
        RTC_I2C_CTRL_CLK_GATE_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_rx_lsb_first(&self) -> RTC_I2C_RX_LSB_FIRST_R {
        RTC_I2C_RX_LSB_FIRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_tx_lsb_first(&self) -> RTC_I2C_TX_LSB_FIRST_R {
        RTC_I2C_TX_LSB_FIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_start(&self) -> RTC_I2C_TRANS_START_R {
        RTC_I2C_TRANS_START_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_ms_mode(&self) -> RTC_I2C_MS_MODE_R {
        RTC_I2C_MS_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_scl_force_out(&self) -> RTC_I2C_SCL_FORCE_OUT_R {
        RTC_I2C_SCL_FORCE_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_sda_force_out(&self) -> RTC_I2C_SDA_FORCE_OUT_R {
        RTC_I2C_SDA_FORCE_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_i2c_clk_en(&mut self) -> RTC_I2C_CLK_EN_W {
        RTC_I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtc_i2c_reset(&mut self) -> RTC_I2C_RESET_W {
        RTC_I2C_RESET_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_i2c_ctrl_clk_gate_en(&mut self) -> RTC_I2C_CTRL_CLK_GATE_EN_W {
        RTC_I2C_CTRL_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtc_i2c_rx_lsb_first(&mut self) -> RTC_I2C_RX_LSB_FIRST_W {
        RTC_I2C_RX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_i2c_tx_lsb_first(&mut self) -> RTC_I2C_TX_LSB_FIRST_W {
        RTC_I2C_TX_LSB_FIRST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtc_i2c_trans_start(&mut self) -> RTC_I2C_TRANS_START_W {
        RTC_I2C_TRANS_START_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtc_i2c_ms_mode(&mut self) -> RTC_I2C_MS_MODE_W {
        RTC_I2C_MS_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rtc_i2c_scl_force_out(&mut self) -> RTC_I2C_SCL_FORCE_OUT_W {
        RTC_I2C_SCL_FORCE_OUT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtc_i2c_sda_force_out(&mut self) -> RTC_I2C_SDA_FORCE_OUT_W {
        RTC_I2C_SDA_FORCE_OUT_W { w: self }
    }
}
