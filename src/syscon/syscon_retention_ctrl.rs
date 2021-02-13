#[doc = "Reader of register SYSCON_RETENTION_CTRL"]
pub type R = crate::R<u32, super::SYSCON_RETENTION_CTRL>;
#[doc = "Writer for register SYSCON_RETENTION_CTRL"]
pub type W = crate::W<u32, super::SYSCON_RETENTION_CTRL>;
#[doc = "Register SYSCON_RETENTION_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_RETENTION_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_NOBYPASS_CPU_ISO_RST`"]
pub type SYSCON_NOBYPASS_CPU_ISO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_NOBYPASS_CPU_ISO_RST`"]
pub struct SYSCON_NOBYPASS_CPU_ISO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_NOBYPASS_CPU_ISO_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_RETENTION_LINK_ADDR`"]
pub type SYSCON_RETENTION_LINK_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_RETENTION_LINK_ADDR`"]
pub struct SYSCON_RETENTION_LINK_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_RETENTION_LINK_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | ((value as u32) & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn syscon_nobypass_cpu_iso_rst(&self) -> SYSCON_NOBYPASS_CPU_ISO_RST_R {
        SYSCON_NOBYPASS_CPU_ISO_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn syscon_retention_link_addr(&self) -> SYSCON_RETENTION_LINK_ADDR_R {
        SYSCON_RETENTION_LINK_ADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn syscon_nobypass_cpu_iso_rst(&mut self) -> SYSCON_NOBYPASS_CPU_ISO_RST_W {
        SYSCON_NOBYPASS_CPU_ISO_RST_W { w: self }
    }
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn syscon_retention_link_addr(&mut self) -> SYSCON_RETENTION_LINK_ADDR_W {
        SYSCON_RETENTION_LINK_ADDR_W { w: self }
    }
}
