#[doc = "Reader of register TIMG_WDTCONFIG0"]
pub type R = crate::R<u32, super::TIMG_WDTCONFIG0>;
#[doc = "Writer for register TIMG_WDTCONFIG0"]
pub type W = crate::W<u32, super::TIMG_WDTCONFIG0>;
#[doc = "Register TIMG_WDTCONFIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMG_WDTCONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMG_WDT_EN`"]
pub type TIMG_WDT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_EN`"]
pub struct TIMG_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_STG0`"]
pub type TIMG_WDT_STG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_STG0`"]
pub struct TIMG_WDT_STG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_STG1`"]
pub type TIMG_WDT_STG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_STG1`"]
pub struct TIMG_WDT_STG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_STG2`"]
pub type TIMG_WDT_STG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_STG2`"]
pub struct TIMG_WDT_STG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_STG3`"]
pub type TIMG_WDT_STG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_STG3`"]
pub struct TIMG_WDT_STG3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_STG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `TIMG_WDT_CONF_UPDATE_EN`"]
pub struct TIMG_WDT_CONF_UPDATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_CONF_UPDATE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_USE_XTAL`"]
pub type TIMG_WDT_USE_XTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_USE_XTAL`"]
pub struct TIMG_WDT_USE_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_USE_XTAL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_CPU_RESET_LENGTH`"]
pub type TIMG_WDT_CPU_RESET_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_CPU_RESET_LENGTH`"]
pub struct TIMG_WDT_CPU_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_CPU_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_SYS_RESET_LENGTH`"]
pub type TIMG_WDT_SYS_RESET_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMG_WDT_SYS_RESET_LENGTH`"]
pub struct TIMG_WDT_SYS_RESET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_SYS_RESET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_FLASHBOOT_MOD_EN`"]
pub type TIMG_WDT_FLASHBOOT_MOD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_FLASHBOOT_MOD_EN`"]
pub struct TIMG_WDT_FLASHBOOT_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_FLASHBOOT_MOD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_PROCPU_RESET_EN`"]
pub type TIMG_WDT_PROCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_PROCPU_RESET_EN`"]
pub struct TIMG_WDT_PROCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_PROCPU_RESET_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMG_WDT_APPCPU_RESET_EN`"]
pub type TIMG_WDT_APPCPU_RESET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG_WDT_APPCPU_RESET_EN`"]
pub struct TIMG_WDT_APPCPU_RESET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG_WDT_APPCPU_RESET_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_wdt_en(&self) -> TIMG_WDT_EN_R {
        TIMG_WDT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn timg_wdt_stg0(&self) -> TIMG_WDT_STG0_R {
        TIMG_WDT_STG0_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn timg_wdt_stg1(&self) -> TIMG_WDT_STG1_R {
        TIMG_WDT_STG1_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn timg_wdt_stg2(&self) -> TIMG_WDT_STG2_R {
        TIMG_WDT_STG2_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn timg_wdt_stg3(&self) -> TIMG_WDT_STG3_R {
        TIMG_WDT_STG3_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timg_wdt_use_xtal(&self) -> TIMG_WDT_USE_XTAL_R {
        TIMG_WDT_USE_XTAL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn timg_wdt_cpu_reset_length(&self) -> TIMG_WDT_CPU_RESET_LENGTH_R {
        TIMG_WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn timg_wdt_sys_reset_length(&self) -> TIMG_WDT_SYS_RESET_LENGTH_R {
        TIMG_WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn timg_wdt_flashboot_mod_en(&self) -> TIMG_WDT_FLASHBOOT_MOD_EN_R {
        TIMG_WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timg_wdt_procpu_reset_en(&self) -> TIMG_WDT_PROCPU_RESET_EN_R {
        TIMG_WDT_PROCPU_RESET_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_wdt_appcpu_reset_en(&self) -> TIMG_WDT_APPCPU_RESET_EN_R {
        TIMG_WDT_APPCPU_RESET_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn timg_wdt_en(&mut self) -> TIMG_WDT_EN_W {
        TIMG_WDT_EN_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn timg_wdt_stg0(&mut self) -> TIMG_WDT_STG0_W {
        TIMG_WDT_STG0_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn timg_wdt_stg1(&mut self) -> TIMG_WDT_STG1_W {
        TIMG_WDT_STG1_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn timg_wdt_stg2(&mut self) -> TIMG_WDT_STG2_W {
        TIMG_WDT_STG2_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn timg_wdt_stg3(&mut self) -> TIMG_WDT_STG3_W {
        TIMG_WDT_STG3_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timg_wdt_conf_update_en(&mut self) -> TIMG_WDT_CONF_UPDATE_EN_W {
        TIMG_WDT_CONF_UPDATE_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timg_wdt_use_xtal(&mut self) -> TIMG_WDT_USE_XTAL_W {
        TIMG_WDT_USE_XTAL_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn timg_wdt_cpu_reset_length(&mut self) -> TIMG_WDT_CPU_RESET_LENGTH_W {
        TIMG_WDT_CPU_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn timg_wdt_sys_reset_length(&mut self) -> TIMG_WDT_SYS_RESET_LENGTH_W {
        TIMG_WDT_SYS_RESET_LENGTH_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn timg_wdt_flashboot_mod_en(&mut self) -> TIMG_WDT_FLASHBOOT_MOD_EN_W {
        TIMG_WDT_FLASHBOOT_MOD_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timg_wdt_procpu_reset_en(&mut self) -> TIMG_WDT_PROCPU_RESET_EN_W {
        TIMG_WDT_PROCPU_RESET_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn timg_wdt_appcpu_reset_en(&mut self) -> TIMG_WDT_APPCPU_RESET_EN_W {
        TIMG_WDT_APPCPU_RESET_EN_W { w: self }
    }
}
