#[doc = "Reader of register RMT_CH2CONF1"]
pub type R = crate::R<u32, super::RMT_CH2CONF1>;
#[doc = "Writer for register RMT_CH2CONF1"]
pub type W = crate::W<u32, super::RMT_CH2CONF1>;
#[doc = "Register RMT_CH2CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH2CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RMT_CONF_UPDATE_CH2`"]
pub struct RMT_CONF_UPDATE_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CONF_UPDATE_CH2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `RMT_AFIFO_RST_CH2`"]
pub struct RMT_AFIFO_RST_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_AFIFO_RST_CH2_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_RX_WRAP_EN_CH2`"]
pub type RMT_MEM_RX_WRAP_EN_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_RX_WRAP_EN_CH2`"]
pub struct RMT_MEM_RX_WRAP_EN_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_RX_WRAP_EN_CH2_W<'a> {
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
#[doc = "Reader of field `RMT_RX_FILTER_THRES_CH2`"]
pub type RMT_RX_FILTER_THRES_CH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_RX_FILTER_THRES_CH2`"]
pub struct RMT_RX_FILTER_THRES_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_FILTER_THRES_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = "Reader of field `RMT_RX_FILTER_EN_CH2`"]
pub type RMT_RX_FILTER_EN_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_RX_FILTER_EN_CH2`"]
pub struct RMT_RX_FILTER_EN_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_FILTER_EN_CH2_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_OWNER_CH2`"]
pub type RMT_MEM_OWNER_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_OWNER_CH2`"]
pub struct RMT_MEM_OWNER_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_OWNER_CH2_W<'a> {
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
#[doc = "Write proxy for field `RMT_APB_MEM_RST_CH2`"]
pub struct RMT_APB_MEM_RST_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_RST_CH2_W<'a> {
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
#[doc = "Write proxy for field `RMT_MEM_WR_RST_CH2`"]
pub struct RMT_MEM_WR_RST_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_WR_RST_CH2_W<'a> {
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
#[doc = "Reader of field `RMT_RX_EN_CH2`"]
pub type RMT_RX_EN_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_RX_EN_CH2`"]
pub struct RMT_RX_EN_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_EN_CH2_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_mem_rx_wrap_en_ch2(&self) -> RMT_MEM_RX_WRAP_EN_CH2_R {
        RMT_MEM_RX_WRAP_EN_CH2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn rmt_rx_filter_thres_ch2(&self) -> RMT_RX_FILTER_THRES_CH2_R {
        RMT_RX_FILTER_THRES_CH2_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_rx_filter_en_ch2(&self) -> RMT_RX_FILTER_EN_CH2_R {
        RMT_RX_FILTER_EN_CH2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_mem_owner_ch2(&self) -> RMT_MEM_OWNER_CH2_R {
        RMT_MEM_OWNER_CH2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_rx_en_ch2(&self) -> RMT_RX_EN_CH2_R {
        RMT_RX_EN_CH2_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rmt_conf_update_ch2(&mut self) -> RMT_CONF_UPDATE_CH2_W {
        RMT_CONF_UPDATE_CH2_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rmt_afifo_rst_ch2(&mut self) -> RMT_AFIFO_RST_CH2_W {
        RMT_AFIFO_RST_CH2_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_mem_rx_wrap_en_ch2(&mut self) -> RMT_MEM_RX_WRAP_EN_CH2_W {
        RMT_MEM_RX_WRAP_EN_CH2_W { w: self }
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn rmt_rx_filter_thres_ch2(&mut self) -> RMT_RX_FILTER_THRES_CH2_W {
        RMT_RX_FILTER_THRES_CH2_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_rx_filter_en_ch2(&mut self) -> RMT_RX_FILTER_EN_CH2_W {
        RMT_RX_FILTER_EN_CH2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_mem_owner_ch2(&mut self) -> RMT_MEM_OWNER_CH2_W {
        RMT_MEM_OWNER_CH2_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_apb_mem_rst_ch2(&mut self) -> RMT_APB_MEM_RST_CH2_W {
        RMT_APB_MEM_RST_CH2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_mem_wr_rst_ch2(&mut self) -> RMT_MEM_WR_RST_CH2_W {
        RMT_MEM_WR_RST_CH2_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_rx_en_ch2(&mut self) -> RMT_RX_EN_CH2_W {
        RMT_RX_EN_CH2_W { w: self }
    }
}
