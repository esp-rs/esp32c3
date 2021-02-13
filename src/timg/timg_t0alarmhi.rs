#[doc = "Reader of register TIMG_T0ALARMHI"]
pub type R = crate::R<u32, super::TIMG_T0ALARMHI>;
#[doc = "Writer for register TIMG_T0ALARMHI"]
pub type W = crate::W<u32, super::TIMG_T0ALARMHI>;
#[doc = "Register TIMG_T0ALARMHI `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_T0ALARMHI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_T0_ALARM_HI`"]
pub type TIMG_T0_ALARM_HI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TIMG_T0_ALARM_HI`"]
pub struct TIMG_T0_ALARM_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_T0_ALARM_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn timg_t0_alarm_hi(&self) -> TIMG_T0_ALARM_HI_R {
        TIMG_T0_ALARM_HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn timg_t0_alarm_hi(&mut self) -> TIMG_T0_ALARM_HI_W {
        TIMG_T0_ALARM_HI_W { w: self }
    }
}
