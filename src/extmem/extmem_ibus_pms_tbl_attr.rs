#[doc = "Reader of register EXTMEM_IBUS_PMS_TBL_ATTR"]
pub type R = crate::R<u32, super::EXTMEM_IBUS_PMS_TBL_ATTR>;
#[doc = "Writer for register EXTMEM_IBUS_PMS_TBL_ATTR"]
pub type W = crate::W<u32, super::EXTMEM_IBUS_PMS_TBL_ATTR>;
#[doc = "Register EXTMEM_IBUS_PMS_TBL_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_IBUS_PMS_TBL_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_IBUS_PMS_SCT2_ATTR`"]
pub type EXTMEM_IBUS_PMS_SCT2_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_IBUS_PMS_SCT2_ATTR`"]
pub struct EXTMEM_IBUS_PMS_SCT2_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_PMS_SCT2_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_IBUS_PMS_SCT1_ATTR`"]
pub type EXTMEM_IBUS_PMS_SCT1_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTMEM_IBUS_PMS_SCT1_ATTR`"]
pub struct EXTMEM_IBUS_PMS_SCT1_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_PMS_SCT1_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn extmem_ibus_pms_sct2_attr(&self) -> EXTMEM_IBUS_PMS_SCT2_ATTR_R {
        EXTMEM_IBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn extmem_ibus_pms_sct1_attr(&self) -> EXTMEM_IBUS_PMS_SCT1_ATTR_R {
        EXTMEM_IBUS_PMS_SCT1_ATTR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn extmem_ibus_pms_sct2_attr(&mut self) -> EXTMEM_IBUS_PMS_SCT2_ATTR_W {
        EXTMEM_IBUS_PMS_SCT2_ATTR_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn extmem_ibus_pms_sct1_attr(&mut self) -> EXTMEM_IBUS_PMS_SCT1_ATTR_W {
        EXTMEM_IBUS_PMS_SCT1_ATTR_W { w: self }
    }
}
