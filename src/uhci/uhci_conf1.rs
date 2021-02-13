#[doc = "Reader of register UHCI_CONF1"]
pub type R = crate::R<u32, super::UHCI_CONF1>;
#[doc = "Writer for register UHCI_CONF1"]
pub type W = crate::W<u32, super::UHCI_CONF1>;
#[doc = "Register UHCI_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_SW_START`"]
pub type UHCI_SW_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SW_START`"]
pub struct UHCI_SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SW_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `UHCI_WAIT_SW_START`"]
pub type UHCI_WAIT_SW_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_WAIT_SW_START`"]
pub struct UHCI_WAIT_SW_START_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_WAIT_SW_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TX_ACK_NUM_RE`"]
pub type UHCI_TX_ACK_NUM_RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_ACK_NUM_RE`"]
pub struct UHCI_TX_ACK_NUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_ACK_NUM_RE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UHCI_TX_CHECK_SUM_RE`"]
pub type UHCI_TX_CHECK_SUM_RE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_TX_CHECK_SUM_RE`"]
pub struct UHCI_TX_CHECK_SUM_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_TX_CHECK_SUM_RE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `UHCI_SAVE_HEAD`"]
pub type UHCI_SAVE_HEAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_SAVE_HEAD`"]
pub struct UHCI_SAVE_HEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SAVE_HEAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UHCI_CRC_DISABLE`"]
pub type UHCI_CRC_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CRC_DISABLE`"]
pub struct UHCI_CRC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CRC_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UHCI_CHECK_SEQ_EN`"]
pub type UHCI_CHECK_SEQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CHECK_SEQ_EN`"]
pub struct UHCI_CHECK_SEQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CHECK_SEQ_EN_W<'a> {
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
#[doc = "Reader of field `UHCI_CHECK_SUM_EN`"]
pub type UHCI_CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UHCI_CHECK_SUM_EN`"]
pub struct UHCI_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_CHECK_SUM_EN_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_sw_start(&self) -> UHCI_SW_START_R {
        UHCI_SW_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_wait_sw_start(&self) -> UHCI_WAIT_SW_START_R {
        UHCI_WAIT_SW_START_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_tx_ack_num_re(&self) -> UHCI_TX_ACK_NUM_RE_R {
        UHCI_TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_tx_check_sum_re(&self) -> UHCI_TX_CHECK_SUM_RE_R {
        UHCI_TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_save_head(&self) -> UHCI_SAVE_HEAD_R {
        UHCI_SAVE_HEAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_crc_disable(&self) -> UHCI_CRC_DISABLE_R {
        UHCI_CRC_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_check_seq_en(&self) -> UHCI_CHECK_SEQ_EN_R {
        UHCI_CHECK_SEQ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_check_sum_en(&self) -> UHCI_CHECK_SUM_EN_R {
        UHCI_CHECK_SUM_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci_sw_start(&mut self) -> UHCI_SW_START_W {
        UHCI_SW_START_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uhci_wait_sw_start(&mut self) -> UHCI_WAIT_SW_START_W {
        UHCI_WAIT_SW_START_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uhci_tx_ack_num_re(&mut self) -> UHCI_TX_ACK_NUM_RE_W {
        UHCI_TX_ACK_NUM_RE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uhci_tx_check_sum_re(&mut self) -> UHCI_TX_CHECK_SUM_RE_W {
        UHCI_TX_CHECK_SUM_RE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uhci_save_head(&mut self) -> UHCI_SAVE_HEAD_W {
        UHCI_SAVE_HEAD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uhci_crc_disable(&mut self) -> UHCI_CRC_DISABLE_W {
        UHCI_CRC_DISABLE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uhci_check_seq_en(&mut self) -> UHCI_CHECK_SEQ_EN_W {
        UHCI_CHECK_SEQ_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uhci_check_sum_en(&mut self) -> UHCI_CHECK_SUM_EN_W {
        UHCI_CHECK_SUM_EN_W { w: self }
    }
}
