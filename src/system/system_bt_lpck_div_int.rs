#[doc = "Reader of register SYSTEM_BT_LPCK_DIV_INT"]
pub type R = crate::R<u32, super::SYSTEM_BT_LPCK_DIV_INT>;
#[doc = "Writer for register SYSTEM_BT_LPCK_DIV_INT"]
pub type W = crate::W<u32, super::SYSTEM_BT_LPCK_DIV_INT>;
#[doc = "Register SYSTEM_BT_LPCK_DIV_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_BT_LPCK_DIV_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_BT_LPCK_DIV_NUM`"]
pub type SYSTEM_BT_LPCK_DIV_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSTEM_BT_LPCK_DIV_NUM`"]
pub struct SYSTEM_BT_LPCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_BT_LPCK_DIV_NUM_W<'a> {
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
    pub fn system_bt_lpck_div_num(&self) -> SYSTEM_BT_LPCK_DIV_NUM_R {
        SYSTEM_BT_LPCK_DIV_NUM_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn system_bt_lpck_div_num(&mut self) -> SYSTEM_BT_LPCK_DIV_NUM_W {
        SYSTEM_BT_LPCK_DIV_NUM_W { w: self }
    }
}
