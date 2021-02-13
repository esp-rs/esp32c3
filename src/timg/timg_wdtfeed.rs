#[doc = "Writer for register TIMG_WDTFEED"]
pub type W = crate::W<u32, super::TIMG_WDTFEED>;
#[doc = "Register TIMG_WDTFEED `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_WDTFEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TIMG_WDT_FEED`"]
pub struct TIMG_WDT_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_FEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timg_wdt_feed(&mut self) -> TIMG_WDT_FEED_W {
        TIMG_WDT_FEED_W { w: self }
    }
}
