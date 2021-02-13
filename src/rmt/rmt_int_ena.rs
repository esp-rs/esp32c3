#[doc = "Reader of register RMT_INT_ENA"]
pub type R = crate::R<u32, super::RMT_INT_ENA>;
#[doc = "Writer for register RMT_INT_ENA"]
pub type W = crate::W<u32, super::RMT_INT_ENA>;
#[doc = "Register RMT_INT_ENA `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_INT_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CH1_TX_LOOP_INT_ENA`"]
pub type RMT_CH1_TX_LOOP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_TX_LOOP_INT_ENA`"]
pub struct RMT_CH1_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH0_TX_LOOP_INT_ENA`"]
pub type RMT_CH0_TX_LOOP_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_TX_LOOP_INT_ENA`"]
pub struct RMT_CH0_TX_LOOP_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_LOOP_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH3_RX_THR_EVENT_INT_ENA`"]
pub type RMT_CH3_RX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_RX_THR_EVENT_INT_ENA`"]
pub struct RMT_CH3_RX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_RX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH2_RX_THR_EVENT_INT_ENA`"]
pub type RMT_CH2_RX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_RX_THR_EVENT_INT_ENA`"]
pub struct RMT_CH2_RX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_RX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH1_TX_THR_EVENT_INT_ENA`"]
pub type RMT_CH1_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_TX_THR_EVENT_INT_ENA`"]
pub struct RMT_CH1_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_THR_EVENT_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH0_TX_THR_EVENT_INT_ENA`"]
pub type RMT_CH0_TX_THR_EVENT_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_TX_THR_EVENT_INT_ENA`"]
pub struct RMT_CH0_TX_THR_EVENT_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_THR_EVENT_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_ERR_INT_ENA`"]
pub type RMT_CH3_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_ERR_INT_ENA`"]
pub struct RMT_CH3_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_ERR_INT_ENA`"]
pub type RMT_CH2_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_ERR_INT_ENA`"]
pub struct RMT_CH2_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_ERR_INT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RMT_CH1_ERR_INT_ENA`"]
pub type RMT_CH1_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_ERR_INT_ENA`"]
pub struct RMT_CH1_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_ERR_INT_ENA`"]
pub type RMT_CH0_ERR_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_ERR_INT_ENA`"]
pub struct RMT_CH0_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_ERR_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH3_RX_END_INT_ENA`"]
pub type RMT_CH3_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH3_RX_END_INT_ENA`"]
pub struct RMT_CH3_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH2_RX_END_INT_ENA`"]
pub type RMT_CH2_RX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH2_RX_END_INT_ENA`"]
pub struct RMT_CH2_RX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_RX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH1_TX_END_INT_ENA`"]
pub type RMT_CH1_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH1_TX_END_INT_ENA`"]
pub struct RMT_CH1_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_END_INT_ENA_W<'a> {
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
#[doc = "Reader of field `RMT_CH0_TX_END_INT_ENA`"]
pub type RMT_CH0_TX_END_INT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CH0_TX_END_INT_ENA`"]
pub struct RMT_CH0_TX_END_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_END_INT_ENA_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_ch1_tx_loop_int_ena(&self) -> RMT_CH1_TX_LOOP_INT_ENA_R {
        RMT_CH1_TX_LOOP_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rmt_ch0_tx_loop_int_ena(&self) -> RMT_CH0_TX_LOOP_INT_ENA_R {
        RMT_CH0_TX_LOOP_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rmt_ch3_rx_thr_event_int_ena(&self) -> RMT_CH3_RX_THR_EVENT_INT_ENA_R {
        RMT_CH3_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rmt_ch2_rx_thr_event_int_ena(&self) -> RMT_CH2_RX_THR_EVENT_INT_ENA_R {
        RMT_CH2_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_ch1_tx_thr_event_int_ena(&self) -> RMT_CH1_TX_THR_EVENT_INT_ENA_R {
        RMT_CH1_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmt_ch0_tx_thr_event_int_ena(&self) -> RMT_CH0_TX_THR_EVENT_INT_ENA_R {
        RMT_CH0_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rmt_ch3_err_int_ena(&self) -> RMT_CH3_ERR_INT_ENA_R {
        RMT_CH3_ERR_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rmt_ch2_err_int_ena(&self) -> RMT_CH2_ERR_INT_ENA_R {
        RMT_CH2_ERR_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rmt_ch1_err_int_ena(&self) -> RMT_CH1_ERR_INT_ENA_R {
        RMT_CH1_ERR_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_ch0_err_int_ena(&self) -> RMT_CH0_ERR_INT_ENA_R {
        RMT_CH0_ERR_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_ch3_rx_end_int_ena(&self) -> RMT_CH3_RX_END_INT_ENA_R {
        RMT_CH3_RX_END_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_ch2_rx_end_int_ena(&self) -> RMT_CH2_RX_END_INT_ENA_R {
        RMT_CH2_RX_END_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_ch1_tx_end_int_ena(&self) -> RMT_CH1_TX_END_INT_ENA_R {
        RMT_CH1_TX_END_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_ch0_tx_end_int_ena(&self) -> RMT_CH0_TX_END_INT_ENA_R {
        RMT_CH0_TX_END_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_ch1_tx_loop_int_ena(&mut self) -> RMT_CH1_TX_LOOP_INT_ENA_W {
        RMT_CH1_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rmt_ch0_tx_loop_int_ena(&mut self) -> RMT_CH0_TX_LOOP_INT_ENA_W {
        RMT_CH0_TX_LOOP_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rmt_ch3_rx_thr_event_int_ena(&mut self) -> RMT_CH3_RX_THR_EVENT_INT_ENA_W {
        RMT_CH3_RX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rmt_ch2_rx_thr_event_int_ena(&mut self) -> RMT_CH2_RX_THR_EVENT_INT_ENA_W {
        RMT_CH2_RX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_ch1_tx_thr_event_int_ena(&mut self) -> RMT_CH1_TX_THR_EVENT_INT_ENA_W {
        RMT_CH1_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmt_ch0_tx_thr_event_int_ena(&mut self) -> RMT_CH0_TX_THR_EVENT_INT_ENA_W {
        RMT_CH0_TX_THR_EVENT_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rmt_ch3_err_int_ena(&mut self) -> RMT_CH3_ERR_INT_ENA_W {
        RMT_CH3_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rmt_ch2_err_int_ena(&mut self) -> RMT_CH2_ERR_INT_ENA_W {
        RMT_CH2_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rmt_ch1_err_int_ena(&mut self) -> RMT_CH1_ERR_INT_ENA_W {
        RMT_CH1_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_ch0_err_int_ena(&mut self) -> RMT_CH0_ERR_INT_ENA_W {
        RMT_CH0_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_ch3_rx_end_int_ena(&mut self) -> RMT_CH3_RX_END_INT_ENA_W {
        RMT_CH3_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_ch2_rx_end_int_ena(&mut self) -> RMT_CH2_RX_END_INT_ENA_W {
        RMT_CH2_RX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_ch1_tx_end_int_ena(&mut self) -> RMT_CH1_TX_END_INT_ENA_W {
        RMT_CH1_TX_END_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_ch0_tx_end_int_ena(&mut self) -> RMT_CH0_TX_END_INT_ENA_W {
        RMT_CH0_TX_END_INT_ENA_W { w: self }
    }
}
