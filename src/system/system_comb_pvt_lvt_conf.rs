#[doc = "Reader of register SYSTEM_COMB_PVT_LVT_CONF"]
pub type R = crate::R<u32, super::SYSTEM_COMB_PVT_LVT_CONF>;
#[doc = "Writer for register SYSTEM_COMB_PVT_LVT_CONF"]
pub type W = crate::W<u32, super::SYSTEM_COMB_PVT_LVT_CONF>;
#[doc = "Register SYSTEM_COMB_PVT_LVT_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_COMB_PVT_LVT_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_COMB_PVT_MONITOR_EN_LVT`"]
pub type SYSTEM_COMB_PVT_MONITOR_EN_LVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_COMB_PVT_MONITOR_EN_LVT`"]
pub struct SYSTEM_COMB_PVT_MONITOR_EN_LVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_COMB_PVT_MONITOR_EN_LVT_W<'a> {
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
#[doc = "Write proxy for field `SYSTEM_COMB_ERR_CNT_CLR_LVT`"]
pub struct SYSTEM_COMB_ERR_CNT_CLR_LVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_COMB_ERR_CNT_CLR_LVT_W<'a> {
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
#[doc = "Reader of field `SYSTEM_COMB_PATH_LEN_LVT`"]
pub type SYSTEM_COMB_PATH_LEN_LVT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_COMB_PATH_LEN_LVT`"]
pub struct SYSTEM_COMB_PATH_LEN_LVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_COMB_PATH_LEN_LVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_comb_pvt_monitor_en_lvt(&self) -> SYSTEM_COMB_PVT_MONITOR_EN_LVT_R {
        SYSTEM_COMB_PVT_MONITOR_EN_LVT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn system_comb_path_len_lvt(&self) -> SYSTEM_COMB_PATH_LEN_LVT_R {
        SYSTEM_COMB_PATH_LEN_LVT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn system_comb_pvt_monitor_en_lvt(&mut self) -> SYSTEM_COMB_PVT_MONITOR_EN_LVT_W {
        SYSTEM_COMB_PVT_MONITOR_EN_LVT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_comb_err_cnt_clr_lvt(&mut self) -> SYSTEM_COMB_ERR_CNT_CLR_LVT_W {
        SYSTEM_COMB_ERR_CNT_CLR_LVT_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn system_comb_path_len_lvt(&mut self) -> SYSTEM_COMB_PATH_LEN_LVT_W {
        SYSTEM_COMB_PATH_LEN_LVT_W { w: self }
    }
}
