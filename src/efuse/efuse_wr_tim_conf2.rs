#[doc = "Reader of register EFUSE_WR_TIM_CONF2"]
pub type R = crate::R<u32, super::EFUSE_WR_TIM_CONF2>;
#[doc = "Writer for register EFUSE_WR_TIM_CONF2"]
pub type W = crate::W<u32, super::EFUSE_WR_TIM_CONF2>;
#[doc = "Register EFUSE_WR_TIM_CONF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_WR_TIM_CONF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_PWR_OFF_NUM`"]
pub type EFUSE_PWR_OFF_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_PWR_OFF_NUM`"]
pub struct EFUSE_PWR_OFF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PWR_OFF_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_pwr_off_num(&self) -> EFUSE_PWR_OFF_NUM_R {
        EFUSE_PWR_OFF_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_pwr_off_num(&mut self) -> EFUSE_PWR_OFF_NUM_W {
        EFUSE_PWR_OFF_NUM_W { w: self }
    }
}
