#[doc = "Reader of register EXTMEM_DBUS_PMS_TBL_ATTR"]
pub type R = crate::R<u32, super::EXTMEM_DBUS_PMS_TBL_ATTR>;
#[doc = "Writer for register EXTMEM_DBUS_PMS_TBL_ATTR"]
pub type W = crate::W<u32, super::EXTMEM_DBUS_PMS_TBL_ATTR>;
#[doc = "Register EXTMEM_DBUS_PMS_TBL_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_DBUS_PMS_TBL_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_DBUS_PMS_SCT2_ATTR`"]
pub type EXTMEM_DBUS_PMS_SCT2_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_DBUS_PMS_SCT2_ATTR`"]
pub struct EXTMEM_DBUS_PMS_SCT2_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DBUS_PMS_SCT2_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_DBUS_PMS_SCT1_ATTR`"]
pub type EXTMEM_DBUS_PMS_SCT1_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_DBUS_PMS_SCT1_ATTR`"]
pub struct EXTMEM_DBUS_PMS_SCT1_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_DBUS_PMS_SCT1_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn extmem_dbus_pms_sct2_attr(&self) -> EXTMEM_DBUS_PMS_SCT2_ATTR_R {
        EXTMEM_DBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn extmem_dbus_pms_sct1_attr(&self) -> EXTMEM_DBUS_PMS_SCT1_ATTR_R {
        EXTMEM_DBUS_PMS_SCT1_ATTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn extmem_dbus_pms_sct2_attr(&mut self) -> EXTMEM_DBUS_PMS_SCT2_ATTR_W {
        EXTMEM_DBUS_PMS_SCT2_ATTR_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn extmem_dbus_pms_sct1_attr(&mut self) -> EXTMEM_DBUS_PMS_SCT1_ATTR_W {
        EXTMEM_DBUS_PMS_SCT1_ATTR_W { w: self }
    }
}
