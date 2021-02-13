#[doc = "Reader of register EFUSE_WR_TIM_CONF1"]
pub type R = crate::R<u32, super::EFUSE_WR_TIM_CONF1>;
#[doc = "Writer for register EFUSE_WR_TIM_CONF1"]
pub type W = crate::W<u32, super::EFUSE_WR_TIM_CONF1>;
#[doc = "Register EFUSE_WR_TIM_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_WR_TIM_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_PWR_ON_NUM`"]
pub type EFUSE_PWR_ON_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_PWR_ON_NUM`"]
pub struct EFUSE_PWR_ON_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_PWR_ON_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn efuse_pwr_on_num(&self) -> EFUSE_PWR_ON_NUM_R {
        EFUSE_PWR_ON_NUM_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:23"]
    #[inline(always)]
    pub fn efuse_pwr_on_num(&mut self) -> EFUSE_PWR_ON_NUM_W {
        EFUSE_PWR_ON_NUM_W { w: self }
    }
}
