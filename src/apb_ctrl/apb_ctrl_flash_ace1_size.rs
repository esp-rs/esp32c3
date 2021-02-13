#[doc = "Reader of register APB_CTRL_FLASH_ACE1_SIZE"]
pub type R = crate::R<u32, super::APB_CTRL_FLASH_ACE1_SIZE>;
#[doc = "Writer for register APB_CTRL_FLASH_ACE1_SIZE"]
pub type W = crate::W<u32, super::APB_CTRL_FLASH_ACE1_SIZE>;
#[doc = "Register APB_CTRL_FLASH_ACE1_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_FLASH_ACE1_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_FLASH_ACE1_SIZE`"]
pub type APB_CTRL_FLASH_ACE1_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APB_CTRL_FLASH_ACE1_SIZE`"]
pub struct APB_CTRL_FLASH_ACE1_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_FLASH_ACE1_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn apb_ctrl_flash_ace1_size(&self) -> APB_CTRL_FLASH_ACE1_SIZE_R {
        APB_CTRL_FLASH_ACE1_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn apb_ctrl_flash_ace1_size(&mut self) -> APB_CTRL_FLASH_ACE1_SIZE_W {
        APB_CTRL_FLASH_ACE1_SIZE_W { w: self }
    }
}
