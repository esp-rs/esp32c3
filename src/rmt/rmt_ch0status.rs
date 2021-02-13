#[doc = "Reader of register RMT_CH0STATUS"]
pub type R = crate::R<u32, super::RMT_CH0STATUS>;
#[doc = "Reader of field `RMT_APB_MEM_RADDR_CH0`"]
pub type RMT_APB_MEM_RADDR_CH0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RMT_APB_MEM_WR_ERR_CH0`"]
pub type RMT_APB_MEM_WR_ERR_CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_MEM_EMPTY_CH0`"]
pub type RMT_MEM_EMPTY_CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_APB_MEM_RD_ERR_CH0`"]
pub type RMT_APB_MEM_RD_ERR_CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_APB_MEM_WADDR_CH0`"]
pub type RMT_APB_MEM_WADDR_CH0_R = crate::R<u16, u16>;
#[doc = "Reader of field `RMT_STATE_CH0`"]
pub type RMT_STATE_CH0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RMT_MEM_RADDR_EX_CH0`"]
pub type RMT_MEM_RADDR_EX_CH0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rmt_apb_mem_raddr_ch0(&self) -> RMT_APB_MEM_RADDR_CH0_R {
        RMT_APB_MEM_RADDR_CH0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rmt_apb_mem_wr_err_ch0(&self) -> RMT_APB_MEM_WR_ERR_CH0_R {
        RMT_APB_MEM_WR_ERR_CH0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rmt_mem_empty_ch0(&self) -> RMT_MEM_EMPTY_CH0_R {
        RMT_MEM_EMPTY_CH0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rmt_apb_mem_rd_err_ch0(&self) -> RMT_APB_MEM_RD_ERR_CH0_R {
        RMT_APB_MEM_RD_ERR_CH0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn rmt_apb_mem_waddr_ch0(&self) -> RMT_APB_MEM_WADDR_CH0_R {
        RMT_APB_MEM_WADDR_CH0_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn rmt_state_ch0(&self) -> RMT_STATE_CH0_R {
        RMT_STATE_CH0_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rmt_mem_raddr_ex_ch0(&self) -> RMT_MEM_RADDR_EX_CH0_R {
        RMT_MEM_RADDR_EX_CH0_R::new((self.bits & 0x01ff) as u16)
    }
}
