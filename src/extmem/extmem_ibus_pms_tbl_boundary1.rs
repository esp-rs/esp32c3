#[doc = "Reader of register EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
pub type R = crate::R<u32, super::EXTMEM_IBUS_PMS_TBL_BOUNDARY1>;
#[doc = "Writer for register EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
pub type W = crate::W<u32, super::EXTMEM_IBUS_PMS_TBL_BOUNDARY1>;
#[doc = "Register EXTMEM_IBUS_PMS_TBL_BOUNDARY1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_IBUS_PMS_TBL_BOUNDARY1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_IBUS_PMS_BOUNDARY1`"]
pub type EXTMEM_IBUS_PMS_BOUNDARY1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXTMEM_IBUS_PMS_BOUNDARY1`"]
pub struct EXTMEM_IBUS_PMS_BOUNDARY1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_IBUS_PMS_BOUNDARY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn extmem_ibus_pms_boundary1(&self) -> EXTMEM_IBUS_PMS_BOUNDARY1_R {
        EXTMEM_IBUS_PMS_BOUNDARY1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn extmem_ibus_pms_boundary1(&mut self) -> EXTMEM_IBUS_PMS_BOUNDARY1_W {
        EXTMEM_IBUS_PMS_BOUNDARY1_W { w: self }
    }
}
