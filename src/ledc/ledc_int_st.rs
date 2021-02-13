#[doc = "Reader of register LEDC_INT_ST"]
pub type R = crate::R<u32, super::LEDC_INT_ST>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH5_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH5_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH4_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH4_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH3_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH3_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH2_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH1_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_OVF_CNT_LSCH0_INT_ST`"]
pub type LEDC_OVF_CNT_LSCH0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH5_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH4_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH3_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH2_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH1_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_DUTY_CHNG_END_LSCH0_INT_ST`"]
pub type LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_LSTIMER3_OVF_INT_ST`"]
pub type LEDC_LSTIMER3_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_LSTIMER2_OVF_INT_ST`"]
pub type LEDC_LSTIMER2_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_LSTIMER1_OVF_INT_ST`"]
pub type LEDC_LSTIMER1_OVF_INT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEDC_LSTIMER0_OVF_INT_ST`"]
pub type LEDC_LSTIMER0_OVF_INT_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch5_int_st(&self) -> LEDC_OVF_CNT_LSCH5_INT_ST_R {
        LEDC_OVF_CNT_LSCH5_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch4_int_st(&self) -> LEDC_OVF_CNT_LSCH4_INT_ST_R {
        LEDC_OVF_CNT_LSCH4_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch3_int_st(&self) -> LEDC_OVF_CNT_LSCH3_INT_ST_R {
        LEDC_OVF_CNT_LSCH3_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch2_int_st(&self) -> LEDC_OVF_CNT_LSCH2_INT_ST_R {
        LEDC_OVF_CNT_LSCH2_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch1_int_st(&self) -> LEDC_OVF_CNT_LSCH1_INT_ST_R {
        LEDC_OVF_CNT_LSCH1_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ledc_ovf_cnt_lsch0_int_st(&self) -> LEDC_OVF_CNT_LSCH0_INT_ST_R {
        LEDC_OVF_CNT_LSCH0_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch5_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH5_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch4_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH4_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch3_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH3_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch2_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH2_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch1_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH1_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ledc_duty_chng_end_lsch0_int_st(&self) -> LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R {
        LEDC_DUTY_CHNG_END_LSCH0_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ledc_lstimer3_ovf_int_st(&self) -> LEDC_LSTIMER3_OVF_INT_ST_R {
        LEDC_LSTIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ledc_lstimer2_ovf_int_st(&self) -> LEDC_LSTIMER2_OVF_INT_ST_R {
        LEDC_LSTIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ledc_lstimer1_ovf_int_st(&self) -> LEDC_LSTIMER1_OVF_INT_ST_R {
        LEDC_LSTIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ledc_lstimer0_ovf_int_st(&self) -> LEDC_LSTIMER0_OVF_INT_ST_R {
        LEDC_LSTIMER0_OVF_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
