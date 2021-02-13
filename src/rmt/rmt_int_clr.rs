#[doc = "Writer for register RMT_INT_CLR"]
pub type W = crate::W<u32, super::RMT_INT_CLR>;
#[doc = "Register RMT_INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RMT_CH1_TX_LOOP_INT_CLR`"]
pub struct RMT_CH1_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_LOOP_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH0_TX_LOOP_INT_CLR`"]
pub struct RMT_CH0_TX_LOOP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_LOOP_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH3_RX_THR_EVENT_INT_CLR`"]
pub struct RMT_CH3_RX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_RX_THR_EVENT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH2_RX_THR_EVENT_INT_CLR`"]
pub struct RMT_CH2_RX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_RX_THR_EVENT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH1_TX_THR_EVENT_INT_CLR`"]
pub struct RMT_CH1_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_THR_EVENT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH0_TX_THR_EVENT_INT_CLR`"]
pub struct RMT_CH0_TX_THR_EVENT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_THR_EVENT_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH3_ERR_INT_CLR`"]
pub struct RMT_CH3_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH2_ERR_INT_CLR`"]
pub struct RMT_CH2_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH1_ERR_INT_CLR`"]
pub struct RMT_CH1_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH0_ERR_INT_CLR`"]
pub struct RMT_CH0_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_ERR_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH3_RX_END_INT_CLR`"]
pub struct RMT_CH3_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH3_RX_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH2_RX_END_INT_CLR`"]
pub struct RMT_CH2_RX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH2_RX_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH1_TX_END_INT_CLR`"]
pub struct RMT_CH1_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH1_TX_END_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `RMT_CH0_TX_END_INT_CLR`"]
pub struct RMT_CH0_TX_END_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CH0_TX_END_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rmt_ch1_tx_loop_int_clr(&mut self) -> RMT_CH1_TX_LOOP_INT_CLR_W {
        RMT_CH1_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rmt_ch0_tx_loop_int_clr(&mut self) -> RMT_CH0_TX_LOOP_INT_CLR_W {
        RMT_CH0_TX_LOOP_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rmt_ch3_rx_thr_event_int_clr(&mut self) -> RMT_CH3_RX_THR_EVENT_INT_CLR_W {
        RMT_CH3_RX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rmt_ch2_rx_thr_event_int_clr(&mut self) -> RMT_CH2_RX_THR_EVENT_INT_CLR_W {
        RMT_CH2_RX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_ch1_tx_thr_event_int_clr(&mut self) -> RMT_CH1_TX_THR_EVENT_INT_CLR_W {
        RMT_CH1_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rmt_ch0_tx_thr_event_int_clr(&mut self) -> RMT_CH0_TX_THR_EVENT_INT_CLR_W {
        RMT_CH0_TX_THR_EVENT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rmt_ch3_err_int_clr(&mut self) -> RMT_CH3_ERR_INT_CLR_W {
        RMT_CH3_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rmt_ch2_err_int_clr(&mut self) -> RMT_CH2_ERR_INT_CLR_W {
        RMT_CH2_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rmt_ch1_err_int_clr(&mut self) -> RMT_CH1_ERR_INT_CLR_W {
        RMT_CH1_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rmt_ch0_err_int_clr(&mut self) -> RMT_CH0_ERR_INT_CLR_W {
        RMT_CH0_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_ch3_rx_end_int_clr(&mut self) -> RMT_CH3_RX_END_INT_CLR_W {
        RMT_CH3_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_ch2_rx_end_int_clr(&mut self) -> RMT_CH2_RX_END_INT_CLR_W {
        RMT_CH2_RX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_ch1_tx_end_int_clr(&mut self) -> RMT_CH1_TX_END_INT_CLR_W {
        RMT_CH1_TX_END_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_ch0_tx_end_int_clr(&mut self) -> RMT_CH0_TX_END_INT_CLR_W {
        RMT_CH0_TX_END_INT_CLR_W { w: self }
    }
}
