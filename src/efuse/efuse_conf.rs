#[doc = "Reader of register EFUSE_CONF"]
pub type R = crate::R<u32, super::EFUSE_CONF>;
#[doc = "Writer for register EFUSE_CONF"]
pub type W = crate::W<u32, super::EFUSE_CONF>;
#[doc = "Register EFUSE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_OP_CODE`"]
pub type EFUSE_OP_CODE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EFUSE_OP_CODE`"]
pub struct EFUSE_OP_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_OP_CODE_W<'a> {
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
    pub fn efuse_op_code(&self) -> EFUSE_OP_CODE_R {
        EFUSE_OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn efuse_op_code(&mut self) -> EFUSE_OP_CODE_W {
        EFUSE_OP_CODE_W { w: self }
    }
}
