#[doc = "Reader of register RMT_CH1CONF0"]
pub type R = crate::R<u32, super::RMT_CH1CONF0>;
#[doc = "Writer for register RMT_CH1CONF0"]
pub type W = crate::W<u32, super::RMT_CH1CONF0>;
#[doc = "Register RMT_CH1CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH1CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RMT_CONF_UPDATE_CH1`"]
pub struct RMT_CONF_UPDATE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CONF_UPDATE_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `RMT_AFIFO_RST_CH1`"]
pub struct RMT_AFIFO_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_AFIFO_RST_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_OUT_LV_CH1`"]
pub type RMT_CARRIER_OUT_LV_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_OUT_LV_CH1`"]
pub struct RMT_CARRIER_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_OUT_LV_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_EN_CH1`"]
pub type RMT_CARRIER_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_EN_CH1`"]
pub struct RMT_CARRIER_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_EFF_EN_CH1`"]
pub type RMT_CARRIER_EFF_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_EFF_EN_CH1`"]
pub struct RMT_CARRIER_EFF_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_EFF_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_SIZE_CH1`"]
pub type RMT_MEM_SIZE_CH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_MEM_SIZE_CH1`"]
pub struct RMT_MEM_SIZE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_SIZE_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `RMT_DIV_CNT_CH1`"]
pub type RMT_DIV_CNT_CH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_DIV_CNT_CH1`"]
pub struct RMT_DIV_CNT_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_DIV_CNT_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RMT_TX_STOP_CH1`"]
pub type RMT_TX_STOP_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_TX_STOP_CH1`"]
pub struct RMT_TX_STOP_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_STOP_CH1_W<'a> {
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
#[doc = "Reader of field `RMT_IDLE_OUT_EN_CH1`"]
pub type RMT_IDLE_OUT_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_IDLE_OUT_EN_CH1`"]
pub struct RMT_IDLE_OUT_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_OUT_EN_CH1_W<'a> {
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
#[doc = "Reader of field `RMT_IDLE_OUT_LV_CH1`"]
pub type RMT_IDLE_OUT_LV_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_IDLE_OUT_LV_CH1`"]
pub struct RMT_IDLE_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_OUT_LV_CH1_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_TX_WRAP_EN_CH1`"]
pub type RMT_MEM_TX_WRAP_EN_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_TX_WRAP_EN_CH1`"]
pub struct RMT_MEM_TX_WRAP_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_TX_WRAP_EN_CH1_W<'a> {
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
#[doc = "Reader of field `RMT_TX_CONTI_MODE_CH1`"]
pub type RMT_TX_CONTI_MODE_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_TX_CONTI_MODE_CH1`"]
pub struct RMT_TX_CONTI_MODE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_CONTI_MODE_CH1_W<'a> {
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
#[doc = "Write proxy for field `RMT_APB_MEM_RST_CH1`"]
pub struct RMT_APB_MEM_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_RST_CH1_W<'a> {
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
#[doc = "Write proxy for field `RMT_MEM_RD_RST_CH1`"]
pub struct RMT_MEM_RD_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_RD_RST_CH1_W<'a> {
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
#[doc = "Write proxy for field `RMT_TX_START_CH1`"]
pub struct RMT_TX_START_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_START_CH1_W<'a> {
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
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch1(&self) -> RMT_CARRIER_OUT_LV_CH1_R {
        RMT_CARRIER_OUT_LV_CH1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rmt_carrier_en_ch1(&self) -> RMT_CARRIER_EN_CH1_R {
        RMT_CARRIER_EN_CH1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rmt_carrier_eff_en_ch1(&self) -> RMT_CARRIER_EFF_EN_CH1_R {
        RMT_CARRIER_EFF_EN_CH1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rmt_mem_size_ch1(&self) -> RMT_MEM_SIZE_CH1_R {
        RMT_MEM_SIZE_CH1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rmt_div_cnt_ch1(&self) -> RMT_DIV_CNT_CH1_R {
        RMT_DIV_CNT_CH1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rmt_tx_stop_ch1(&self) -> RMT_TX_STOP_CH1_R {
        RMT_TX_STOP_CH1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rmt_idle_out_en_ch1(&self) -> RMT_IDLE_OUT_EN_CH1_R {
        RMT_IDLE_OUT_EN_CH1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rmt_idle_out_lv_ch1(&self) -> RMT_IDLE_OUT_LV_CH1_R {
        RMT_IDLE_OUT_LV_CH1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_mem_tx_wrap_en_ch1(&self) -> RMT_MEM_TX_WRAP_EN_CH1_R {
        RMT_MEM_TX_WRAP_EN_CH1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_tx_conti_mode_ch1(&self) -> RMT_TX_CONTI_MODE_CH1_R {
        RMT_TX_CONTI_MODE_CH1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rmt_conf_update_ch1(&mut self) -> RMT_CONF_UPDATE_CH1_W {
        RMT_CONF_UPDATE_CH1_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rmt_afifo_rst_ch1(&mut self) -> RMT_AFIFO_RST_CH1_W {
        RMT_AFIFO_RST_CH1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch1(&mut self) -> RMT_CARRIER_OUT_LV_CH1_W {
        RMT_CARRIER_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rmt_carrier_en_ch1(&mut self) -> RMT_CARRIER_EN_CH1_W {
        RMT_CARRIER_EN_CH1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rmt_carrier_eff_en_ch1(&mut self) -> RMT_CARRIER_EFF_EN_CH1_W {
        RMT_CARRIER_EFF_EN_CH1_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rmt_mem_size_ch1(&mut self) -> RMT_MEM_SIZE_CH1_W {
        RMT_MEM_SIZE_CH1_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rmt_div_cnt_ch1(&mut self) -> RMT_DIV_CNT_CH1_W {
        RMT_DIV_CNT_CH1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rmt_tx_stop_ch1(&mut self) -> RMT_TX_STOP_CH1_W {
        RMT_TX_STOP_CH1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rmt_idle_out_en_ch1(&mut self) -> RMT_IDLE_OUT_EN_CH1_W {
        RMT_IDLE_OUT_EN_CH1_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rmt_idle_out_lv_ch1(&mut self) -> RMT_IDLE_OUT_LV_CH1_W {
        RMT_IDLE_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_mem_tx_wrap_en_ch1(&mut self) -> RMT_MEM_TX_WRAP_EN_CH1_W {
        RMT_MEM_TX_WRAP_EN_CH1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_tx_conti_mode_ch1(&mut self) -> RMT_TX_CONTI_MODE_CH1_W {
        RMT_TX_CONTI_MODE_CH1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_apb_mem_rst_ch1(&mut self) -> RMT_APB_MEM_RST_CH1_W {
        RMT_APB_MEM_RST_CH1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_mem_rd_rst_ch1(&mut self) -> RMT_MEM_RD_RST_CH1_W {
        RMT_MEM_RD_RST_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_tx_start_ch1(&mut self) -> RMT_TX_START_CH1_W {
        RMT_TX_START_CH1_W { w: self }
    }
}
