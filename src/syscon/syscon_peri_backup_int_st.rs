#[doc = "Reader of register SYSCON_PERI_BACKUP_INT_ST"]
pub type R = crate::R<u32, super::SYSCON_PERI_BACKUP_INT_ST>;
#[doc = "Reader of field `SYSCON_PERI_BACKUP_ERR_INT_ST`"]
pub type SYSCON_PERI_BACKUP_ERR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSCON_PERI_BACKUP_DONE_INT_ST`"]
pub type SYSCON_PERI_BACKUP_DONE_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn syscon_peri_backup_err_int_st(&self) -> SYSCON_PERI_BACKUP_ERR_INT_ST_R {
        SYSCON_PERI_BACKUP_ERR_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn syscon_peri_backup_done_int_st(&self) -> SYSCON_PERI_BACKUP_DONE_INT_ST_R {
        SYSCON_PERI_BACKUP_DONE_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
