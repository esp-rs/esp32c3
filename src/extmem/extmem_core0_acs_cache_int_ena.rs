#[doc = "Reader of register EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
pub type R = crate::R<u32, super::EXTMEM_CORE0_ACS_CACHE_INT_ENA>;
#[doc = "Writer for register EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
pub type W = crate::W<u32, super::EXTMEM_CORE0_ACS_CACHE_INT_ENA>;
#[doc = "Register EXTMEM_CORE0_ACS_CACHE_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_CORE0_ACS_CACHE_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_WR_IC_INT_ENA`"]
pub type EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_WR_IC_INT_ENA`"]
pub struct EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_REJECT_INT_ENA`"]
pub type EXTMEM_CORE0_DBUS_REJECT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_REJECT_INT_ENA`"]
pub struct EXTMEM_CORE0_DBUS_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_REJECT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA`"]
pub type EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA`"]
pub struct EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_REJECT_INT_ENA`"]
pub type EXTMEM_CORE0_IBUS_REJECT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_REJECT_INT_ENA`"]
pub struct EXTMEM_CORE0_IBUS_REJECT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_REJECT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_WR_IC_INT_ENA`"]
pub type EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_WR_IC_INT_ENA`"]
pub struct EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_W<'a> {
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
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA`"]
pub type EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA`"]
pub struct EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_core0_dbus_wr_ic_int_ena(&self) -> EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_R {
        EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_core0_dbus_reject_int_ena(&self) -> EXTMEM_CORE0_DBUS_REJECT_INT_ENA_R {
        EXTMEM_CORE0_DBUS_REJECT_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_core0_dbus_acs_msk_ic_int_ena(&self) -> EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_R {
        EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_core0_ibus_reject_int_ena(&self) -> EXTMEM_CORE0_IBUS_REJECT_INT_ENA_R {
        EXTMEM_CORE0_IBUS_REJECT_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_core0_ibus_wr_ic_int_ena(&self) -> EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_R {
        EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_core0_ibus_acs_msk_ic_int_ena(&self) -> EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_R {
        EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_core0_dbus_wr_ic_int_ena(&mut self) -> EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_W {
        EXTMEM_CORE0_DBUS_WR_IC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_core0_dbus_reject_int_ena(&mut self) -> EXTMEM_CORE0_DBUS_REJECT_INT_ENA_W {
        EXTMEM_CORE0_DBUS_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_core0_dbus_acs_msk_ic_int_ena(
        &mut self,
    ) -> EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_W {
        EXTMEM_CORE0_DBUS_ACS_MSK_IC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_core0_ibus_reject_int_ena(&mut self) -> EXTMEM_CORE0_IBUS_REJECT_INT_ENA_W {
        EXTMEM_CORE0_IBUS_REJECT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_core0_ibus_wr_ic_int_ena(&mut self) -> EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_W {
        EXTMEM_CORE0_IBUS_WR_IC_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_core0_ibus_acs_msk_ic_int_ena(
        &mut self,
    ) -> EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_W {
        EXTMEM_CORE0_IBUS_ACS_MSK_IC_INT_ENA_W { w: self }
    }
}
