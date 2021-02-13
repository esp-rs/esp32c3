#[doc = "Reader of register EFUSE_STATUS"]
pub type R = crate::R<u32, super::EFUSE_STATUS>;
#[doc = "Reader of field `EFUSE_REPEAT_ERR_CNT`"]
pub type EFUSE_REPEAT_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_OTP_VDDQ_IS_SW`"]
pub type EFUSE_OTP_VDDQ_IS_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_OTP_PGENB_SW`"]
pub type EFUSE_OTP_PGENB_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_OTP_CSB_SW`"]
pub type EFUSE_OTP_CSB_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_OTP_STROBE_SW`"]
pub type EFUSE_OTP_STROBE_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_OTP_VDDQ_C_SYNC2`"]
pub type EFUSE_OTP_VDDQ_C_SYNC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_OTP_LOAD_SW`"]
pub type EFUSE_OTP_LOAD_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_STATE`"]
pub type EFUSE_STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 10:17"]
    #[inline(always)]
    pub fn efuse_repeat_err_cnt(&self) -> EFUSE_REPEAT_ERR_CNT_R {
        EFUSE_REPEAT_ERR_CNT_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_otp_vddq_is_sw(&self) -> EFUSE_OTP_VDDQ_IS_SW_R {
        EFUSE_OTP_VDDQ_IS_SW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_otp_pgenb_sw(&self) -> EFUSE_OTP_PGENB_SW_R {
        EFUSE_OTP_PGENB_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_otp_csb_sw(&self) -> EFUSE_OTP_CSB_SW_R {
        EFUSE_OTP_CSB_SW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn efuse_otp_strobe_sw(&self) -> EFUSE_OTP_STROBE_SW_R {
        EFUSE_OTP_STROBE_SW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn efuse_otp_vddq_c_sync2(&self) -> EFUSE_OTP_VDDQ_C_SYNC2_R {
        EFUSE_OTP_VDDQ_C_SYNC2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn efuse_otp_load_sw(&self) -> EFUSE_OTP_LOAD_SW_R {
        EFUSE_OTP_LOAD_SW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn efuse_state(&self) -> EFUSE_STATE_R {
        EFUSE_STATE_R::new((self.bits & 0x0f) as u8)
    }
}
