#[doc = "Writer for register EXTMEM_CACHE_ACS_CNT_CLR"]
pub type W = crate::W<u32, super::EXTMEM_CACHE_ACS_CNT_CLR>;
#[doc = "Register EXTMEM_CACHE_ACS_CNT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CACHE_ACS_CNT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXTMEM_DBUS_ACS_CNT_CLR`"]
pub struct EXTMEM_DBUS_ACS_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DBUS_ACS_CNT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_IBUS_ACS_CNT_CLR`"]
pub struct EXTMEM_IBUS_ACS_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_ACS_CNT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_dbus_acs_cnt_clr(&mut self) -> EXTMEM_DBUS_ACS_CNT_CLR_W {
        EXTMEM_DBUS_ACS_CNT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_ibus_acs_cnt_clr(&mut self) -> EXTMEM_IBUS_ACS_CNT_CLR_W {
        EXTMEM_IBUS_ACS_CNT_CLR_W { w: self }
    }
}
