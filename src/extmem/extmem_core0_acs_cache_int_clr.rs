#[doc = "Writer for register EXTMEM_CORE0_ACS_CACHE_INT_CLR"]
pub type W = crate::W<u32, super::EXTMEM_CORE0_ACS_CACHE_INT_CLR>;
#[doc = "Register EXTMEM_CORE0_ACS_CACHE_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CORE0_ACS_CACHE_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_WR_IC_INT_CLR`"]
pub struct EXTMEM_CORE0_DBUS_WR_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_WR_IC_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_REJECT_INT_CLR`"]
pub struct EXTMEM_CORE0_DBUS_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_REJECT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_CLR`"]
pub struct EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_REJECT_INT_CLR`"]
pub struct EXTMEM_CORE0_IBUS_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_REJECT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_WR_IC_INT_CLR`"]
pub struct EXTMEM_CORE0_IBUS_WR_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_WR_IC_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_CLR`"]
pub struct EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_core0_dbus_wr_ic_int_clr(&mut self) -> EXTMEM_CORE0_DBUS_WR_IC_INT_CLR_W {
        EXTMEM_CORE0_DBUS_WR_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_core0_dbus_reject_int_clr(&mut self) -> EXTMEM_CORE0_DBUS_REJECT_INT_CLR_W {
        EXTMEM_CORE0_DBUS_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_core0_dbus_acs_msk_ic_int_clr(
        &mut self,
    ) -> EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_CLR_W {
        EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_core0_ibus_reject_int_clr(&mut self) -> EXTMEM_CORE0_IBUS_REJECT_INT_CLR_W {
        EXTMEM_CORE0_IBUS_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_core0_ibus_wr_ic_int_clr(&mut self) -> EXTMEM_CORE0_IBUS_WR_IC_INT_CLR_W {
        EXTMEM_CORE0_IBUS_WR_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_core0_ibus_acs_msk_ic_int_clr(
        &mut self,
    ) -> EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_CLR_W {
        EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_CLR_W { w: self }
    }
}
