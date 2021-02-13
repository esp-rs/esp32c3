#[doc = "Reader of register APB_SARADC_FSM_WAIT"]
pub type R = crate::R<u32, super::APB_SARADC_FSM_WAIT>;
#[doc = "Writer for register APB_SARADC_FSM_WAIT"]
pub type W = crate::W<u32, super::APB_SARADC_FSM_WAIT>;
#[doc = "Register APB_SARADC_FSM_WAIT `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_SARADC_FSM_WAIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_SARADC_STANDBY_WAIT`"]
pub type APB_SARADC_STANDBY_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_STANDBY_WAIT`"]
pub struct APB_SARADC_STANDBY_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_STANDBY_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_RSTB_WAIT`"]
pub type APB_SARADC_RSTB_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_RSTB_WAIT`"]
pub struct APB_SARADC_RSTB_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_RSTB_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `APB_SARADC_XPD_WAIT`"]
pub type APB_SARADC_XPD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_SARADC_XPD_WAIT`"]
pub struct APB_SARADC_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn apb_saradc_standby_wait(&self) -> APB_SARADC_STANDBY_WAIT_R {
        APB_SARADC_STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn apb_saradc_rstb_wait(&self) -> APB_SARADC_RSTB_WAIT_R {
        APB_SARADC_RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_saradc_xpd_wait(&self) -> APB_SARADC_XPD_WAIT_R {
        APB_SARADC_XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn apb_saradc_standby_wait(&mut self) -> APB_SARADC_STANDBY_WAIT_W {
        APB_SARADC_STANDBY_WAIT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn apb_saradc_rstb_wait(&mut self) -> APB_SARADC_RSTB_WAIT_W {
        APB_SARADC_RSTB_WAIT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn apb_saradc_xpd_wait(&mut self) -> APB_SARADC_XPD_WAIT_W {
        APB_SARADC_XPD_WAIT_W { w: self }
    }
}
