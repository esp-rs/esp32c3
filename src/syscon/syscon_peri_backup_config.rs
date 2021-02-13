#[doc = "Reader of register SYSCON_PERI_BACKUP_CONFIG"]
pub type R = crate::R<u32, super::SYSCON_PERI_BACKUP_CONFIG>;
#[doc = "Writer for register SYSCON_PERI_BACKUP_CONFIG"]
pub type W = crate::W<u32, super::SYSCON_PERI_BACKUP_CONFIG>;
#[doc = "Register SYSCON_PERI_BACKUP_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_PERI_BACKUP_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_PERI_BACKUP_ENA`"]
pub type SYSCON_PERI_BACKUP_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_ENA`"]
pub struct SYSCON_PERI_BACKUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_ENA_W<'a> {
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
#[doc = "Reader of field `SYSCON_PERI_BACKUP_TO_MEM`"]
pub type SYSCON_PERI_BACKUP_TO_MEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_TO_MEM`"]
pub struct SYSCON_PERI_BACKUP_TO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_TO_MEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_START`"]
pub struct SYSCON_PERI_BACKUP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PERI_BACKUP_SIZE`"]
pub type SYSCON_PERI_BACKUP_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_SIZE`"]
pub struct SYSCON_PERI_BACKUP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PERI_BACKUP_TOUT_THRES`"]
pub type SYSCON_PERI_BACKUP_TOUT_THRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_TOUT_THRES`"]
pub struct SYSCON_PERI_BACKUP_TOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_TOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | (((value as u32) & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PERI_BACKUP_BURST_LIMIT`"]
pub type SYSCON_PERI_BACKUP_BURST_LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_PERI_BACKUP_BURST_LIMIT`"]
pub struct SYSCON_PERI_BACKUP_BURST_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_BACKUP_BURST_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYSCON_PERI_BACKUP_FLOW_ERR`"]
pub type SYSCON_PERI_BACKUP_FLOW_ERR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn syscon_peri_backup_ena(&self) -> SYSCON_PERI_BACKUP_ENA_R {
        SYSCON_PERI_BACKUP_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn syscon_peri_backup_to_mem(&self) -> SYSCON_PERI_BACKUP_TO_MEM_R {
        SYSCON_PERI_BACKUP_TO_MEM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn syscon_peri_backup_size(&self) -> SYSCON_PERI_BACKUP_SIZE_R {
        SYSCON_PERI_BACKUP_SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn syscon_peri_backup_tout_thres(&self) -> SYSCON_PERI_BACKUP_TOUT_THRES_R {
        SYSCON_PERI_BACKUP_TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn syscon_peri_backup_burst_limit(&self) -> SYSCON_PERI_BACKUP_BURST_LIMIT_R {
        SYSCON_PERI_BACKUP_BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn syscon_peri_backup_flow_err(&self) -> SYSCON_PERI_BACKUP_FLOW_ERR_R {
        SYSCON_PERI_BACKUP_FLOW_ERR_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn syscon_peri_backup_ena(&mut self) -> SYSCON_PERI_BACKUP_ENA_W {
        SYSCON_PERI_BACKUP_ENA_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn syscon_peri_backup_to_mem(&mut self) -> SYSCON_PERI_BACKUP_TO_MEM_W {
        SYSCON_PERI_BACKUP_TO_MEM_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn syscon_peri_backup_start(&mut self) -> SYSCON_PERI_BACKUP_START_W {
        SYSCON_PERI_BACKUP_START_W { w: self }
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn syscon_peri_backup_size(&mut self) -> SYSCON_PERI_BACKUP_SIZE_W {
        SYSCON_PERI_BACKUP_SIZE_W { w: self }
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn syscon_peri_backup_tout_thres(&mut self) -> SYSCON_PERI_BACKUP_TOUT_THRES_W {
        SYSCON_PERI_BACKUP_TOUT_THRES_W { w: self }
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn syscon_peri_backup_burst_limit(&mut self) -> SYSCON_PERI_BACKUP_BURST_LIMIT_W {
        SYSCON_PERI_BACKUP_BURST_LIMIT_W { w: self }
    }
}
