#[doc = "Reader of register EFUSE_RD_TIM_CONF"]
pub type R = crate::R<u32, super::EFUSE_RD_TIM_CONF>;
#[doc = "Writer for register EFUSE_RD_TIM_CONF"]
pub type W = crate::W<u32, super::EFUSE_RD_TIM_CONF>;
#[doc = "Register EFUSE_RD_TIM_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_RD_TIM_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_READ_INIT_NUM`"]
pub type EFUSE_READ_INIT_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_READ_INIT_NUM`"]
pub struct EFUSE_READ_INIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_INIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn efuse_read_init_num(&self) -> EFUSE_READ_INIT_NUM_R {
        EFUSE_READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn efuse_read_init_num(&mut self) -> EFUSE_READ_INIT_NUM_W {
        EFUSE_READ_INIT_NUM_W { w: self }
    }
}
