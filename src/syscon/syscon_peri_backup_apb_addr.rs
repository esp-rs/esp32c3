#[doc = "Reader of register SYSCON_PERI_BACKUP_APB_ADDR"]
pub type R = crate::R<u32, super::SYSCON_PERI_BACKUP_APB_ADDR>;
#[doc = "Writer for register SYSCON_PERI_BACKUP_APB_ADDR"]
pub type W = crate::W<u32, super::SYSCON_PERI_BACKUP_APB_ADDR>;
#[doc = "Register SYSCON_PERI_BACKUP_APB_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_PERI_BACKUP_APB_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_BACKUP_APB_START_ADDR`"]
pub type SYSCON_BACKUP_APB_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SYSCON_BACKUP_APB_START_ADDR`"]
pub struct SYSCON_BACKUP_APB_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_BACKUP_APB_START_ADDR_W<'a> {
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
    pub fn syscon_backup_apb_start_addr(&self) -> SYSCON_BACKUP_APB_START_ADDR_R {
        SYSCON_BACKUP_APB_START_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syscon_backup_apb_start_addr(&mut self) -> SYSCON_BACKUP_APB_START_ADDR_W {
        SYSCON_BACKUP_APB_START_ADDR_W { w: self }
    }
}
