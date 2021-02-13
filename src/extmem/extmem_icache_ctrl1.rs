#[doc = "Reader of register EXTMEM_ICACHE_CTRL1"]
pub type R = crate::R<u32, super::EXTMEM_ICACHE_CTRL1>;
#[doc = "Writer for register EXTMEM_ICACHE_CTRL1"]
pub type W = crate::W<u32, super::EXTMEM_ICACHE_CTRL1>;
#[doc = "Register EXTMEM_ICACHE_CTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMEM_ICACHE_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_SHUT_DBUS`"]
pub type EXTMEM_ICACHE_SHUT_DBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_SHUT_DBUS`"]
pub struct EXTMEM_ICACHE_SHUT_DBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SHUT_DBUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EXTMEM_ICACHE_SHUT_IBUS`"]
pub type EXTMEM_ICACHE_SHUT_IBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTMEM_ICACHE_SHUT_IBUS`"]
pub struct EXTMEM_ICACHE_SHUT_IBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMEM_ICACHE_SHUT_IBUS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_shut_dbus(&self) -> EXTMEM_ICACHE_SHUT_DBUS_R {
        EXTMEM_ICACHE_SHUT_DBUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_shut_ibus(&self) -> EXTMEM_ICACHE_SHUT_IBUS_R {
        EXTMEM_ICACHE_SHUT_IBUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_icache_shut_dbus(&mut self) -> EXTMEM_ICACHE_SHUT_DBUS_W {
        EXTMEM_ICACHE_SHUT_DBUS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_icache_shut_ibus(&mut self) -> EXTMEM_ICACHE_SHUT_IBUS_W {
        EXTMEM_ICACHE_SHUT_IBUS_W { w: self }
    }
}
