#[doc = "Reader of register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6>;
#[doc = "Writer for register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6>;
#[doc = "Register SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT`"]
pub type SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT`"]
pub struct SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_systimer(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timergroup1(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_R::new(
            ((self.bits >> 28) & 0x03) as u8,
        )
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timergroup(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_R::new(
            ((self.bits >> 26) & 0x03) as u8,
        )
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_bb(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_ledc(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rmt(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uhci0(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2c_ext0(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_bt(
        &self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_systimer(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_SYSTIMER_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timergroup1(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP1_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_timergroup(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_TIMERGROUP_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_bb(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BB_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_ledc(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_LEDC_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_rmt(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_RMT_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_uhci0(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_UHCI0_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_i2c_ext0(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_I2C_EXT0_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_0_pif_pms_constrain_world_1_bt(
        &mut self,
    ) -> SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W {
        SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_WORLD_1_BT_W { w: self }
    }
}
