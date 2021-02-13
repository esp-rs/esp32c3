#[doc = "Reader of register TIMG_T0UPDATE"]
pub type R = crate::R<u32, super::TIMG_T0UPDATE>;
#[doc = "Writer for register TIMG_T0UPDATE"]
pub type W = crate::W<u32, super::TIMG_T0UPDATE>;
#[doc = "Register TIMG_T0UPDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_T0UPDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T0_UPDATE`"]
pub type TIMG_T0_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_T0_UPDATE`"]
pub struct TIMG_T0_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_UPDATE_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_t0_update(&self) -> TIMG_T0_UPDATE_R {
        TIMG_T0_UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_t0_update(&mut self) -> TIMG_T0_UPDATE_W {
        TIMG_T0_UPDATE_W { w: self }
    }
}
