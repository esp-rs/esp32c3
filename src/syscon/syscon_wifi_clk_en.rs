#[doc = "Reader of register SYSCON_WIFI_CLK_EN"]
pub type R = crate::R<u32, super::SYSCON_WIFI_CLK_EN>;
#[doc = "Writer for register SYSCON_WIFI_CLK_EN"]
pub type W = crate::W<u32, super::SYSCON_WIFI_CLK_EN>;
#[doc = "Register SYSCON_WIFI_CLK_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_WIFI_CLK_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_WIFI_CLK_EN`"]
pub type SYSCON_WIFI_CLK_EN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_WIFI_CLK_EN`"]
pub struct SYSCON_WIFI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_WIFI_CLK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_wifi_clk_en(&self) -> SYSCON_WIFI_CLK_EN_R {
        SYSCON_WIFI_CLK_EN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_wifi_clk_en(&mut self) -> SYSCON_WIFI_CLK_EN_W {
        SYSCON_WIFI_CLK_EN_W { w: self }
    }
}
