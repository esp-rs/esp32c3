#[doc = "Reader of register APB_CTRL_PERI_BACKUP_INT_RAW"]
pub type R = crate::R<u32, super::APB_CTRL_PERI_BACKUP_INT_RAW>;
#[doc = "Reader of field `APB_CTRL_PERI_BACKUP_ERR_INT_RAW`"]
pub type APB_CTRL_PERI_BACKUP_ERR_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Reader of field `APB_CTRL_PERI_BACKUP_DONE_INT_RAW`"]
pub type APB_CTRL_PERI_BACKUP_DONE_INT_RAW_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_peri_backup_err_int_raw(&self) -> APB_CTRL_PERI_BACKUP_ERR_INT_RAW_R {
        APB_CTRL_PERI_BACKUP_ERR_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_peri_backup_done_int_raw(&self) -> APB_CTRL_PERI_BACKUP_DONE_INT_RAW_R {
        APB_CTRL_PERI_BACKUP_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
