#[doc = "Reader of register APB_CTRL_FLASH_ACE1_ATTR"]
pub type R = crate::R<u32, super::APB_CTRL_FLASH_ACE1_ATTR>;
#[doc = "Writer for register APB_CTRL_FLASH_ACE1_ATTR"]
pub type W = crate::W<u32, super::APB_CTRL_FLASH_ACE1_ATTR>;
#[doc = "Register APB_CTRL_FLASH_ACE1_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_FLASH_ACE1_ATTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_FLASH_ACE1_ATTR`"]
pub type APB_CTRL_FLASH_ACE1_ATTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_FLASH_ACE1_ATTR`"]
pub struct APB_CTRL_FLASH_ACE1_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_FLASH_ACE1_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn apb_ctrl_flash_ace1_attr(&self) -> APB_CTRL_FLASH_ACE1_ATTR_R {
        APB_CTRL_FLASH_ACE1_ATTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn apb_ctrl_flash_ace1_attr(&mut self) -> APB_CTRL_FLASH_ACE1_ATTR_W {
        APB_CTRL_FLASH_ACE1_ATTR_W { w: self }
    }
}
