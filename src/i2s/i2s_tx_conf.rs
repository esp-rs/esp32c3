#[doc = "Reader of register I2S_TX_CONF"]
pub type R = crate::R<u32, super::I2S_TX_CONF>;
#[doc = "Writer for register I2S_TX_CONF"]
pub type W = crate::W<u32, super::I2S_TX_CONF>;
#[doc = "Register I2S_TX_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TX_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_SIG_LOOPBACK`"]
pub type I2S_SIG_LOOPBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_SIG_LOOPBACK`"]
pub struct I2S_SIG_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SIG_LOOPBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_CHAN_MOD`"]
pub type I2S_TX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_CHAN_MOD`"]
pub struct I2S_TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_EN`"]
pub type I2S_TX_PDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_PDM_EN`"]
pub struct I2S_TX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_TDM_EN`"]
pub type I2S_TX_TDM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_TDM_EN`"]
pub struct I2S_TX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_TDM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_BIT_ORDER`"]
pub type I2S_TX_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_BIT_ORDER`"]
pub struct I2S_TX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_WS_IDLE_POL`"]
pub type I2S_TX_WS_IDLE_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_WS_IDLE_POL`"]
pub struct I2S_TX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_WS_IDLE_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_24_FILL_EN`"]
pub type I2S_TX_24_FILL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_24_FILL_EN`"]
pub struct I2S_TX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_24_FILL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_LEFT_ALIGN`"]
pub type I2S_TX_LEFT_ALIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_LEFT_ALIGN`"]
pub struct I2S_TX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_LEFT_ALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_STOP_EN`"]
pub type I2S_TX_STOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_STOP_EN`"]
pub struct I2S_TX_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_STOP_EN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_PCM_BYPASS`"]
pub type I2S_TX_PCM_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_PCM_BYPASS`"]
pub struct I2S_TX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PCM_BYPASS_W<'a> {
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
#[doc = "Reader of field `I2S_TX_PCM_CONF`"]
pub type I2S_TX_PCM_CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_PCM_CONF`"]
pub struct I2S_TX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_MONO_FST_VLD`"]
pub type I2S_TX_MONO_FST_VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MONO_FST_VLD`"]
pub struct I2S_TX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MONO_FST_VLD_W<'a> {
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
#[doc = "Reader of field `I2S_TX_UPDATE`"]
pub type I2S_TX_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_UPDATE`"]
pub struct I2S_TX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_UPDATE_W<'a> {
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
#[doc = "Reader of field `I2S_TX_BIG_ENDIAN`"]
pub type I2S_TX_BIG_ENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_BIG_ENDIAN`"]
pub struct I2S_TX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_BIG_ENDIAN_W<'a> {
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
#[doc = "Reader of field `I2S_TX_CHAN_EQUAL`"]
pub type I2S_TX_CHAN_EQUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_CHAN_EQUAL`"]
pub struct I2S_TX_CHAN_EQUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_CHAN_EQUAL_W<'a> {
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
#[doc = "Reader of field `I2S_TX_MONO`"]
pub type I2S_TX_MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_MONO`"]
pub struct I2S_TX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_MONO_W<'a> {
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
#[doc = "Reader of field `I2S_TX_SLAVE_MOD`"]
pub type I2S_TX_SLAVE_MOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_SLAVE_MOD`"]
pub struct I2S_TX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_SLAVE_MOD_W<'a> {
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
#[doc = "Reader of field `I2S_TX_START`"]
pub type I2S_TX_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_TX_START`"]
pub struct I2S_TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_START_W<'a> {
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
#[doc = "Write proxy for field `I2S_TX_FIFO_RESET`"]
pub struct I2S_TX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_FIFO_RESET_W<'a> {
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
#[doc = "Write proxy for field `I2S_TX_RESET`"]
pub struct I2S_TX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_RESET_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn i2s_sig_loopback(&self) -> I2S_SIG_LOOPBACK_R {
        I2S_SIG_LOOPBACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn i2s_tx_chan_mod(&self) -> I2S_TX_CHAN_MOD_R {
        I2S_TX_CHAN_MOD_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s_tx_pdm_en(&self) -> I2S_TX_PDM_EN_R {
        I2S_TX_PDM_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2s_tx_tdm_en(&self) -> I2S_TX_TDM_EN_R {
        I2S_TX_TDM_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2s_tx_bit_order(&self) -> I2S_TX_BIT_ORDER_R {
        I2S_TX_BIT_ORDER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2s_tx_ws_idle_pol(&self) -> I2S_TX_WS_IDLE_POL_R {
        I2S_TX_WS_IDLE_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_tx_24_fill_en(&self) -> I2S_TX_24_FILL_EN_R {
        I2S_TX_24_FILL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2s_tx_left_align(&self) -> I2S_TX_LEFT_ALIGN_R {
        I2S_TX_LEFT_ALIGN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_tx_stop_en(&self) -> I2S_TX_STOP_EN_R {
        I2S_TX_STOP_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_tx_pcm_bypass(&self) -> I2S_TX_PCM_BYPASS_R {
        I2S_TX_PCM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn i2s_tx_pcm_conf(&self) -> I2S_TX_PCM_CONF_R {
        I2S_TX_PCM_CONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_tx_mono_fst_vld(&self) -> I2S_TX_MONO_FST_VLD_R {
        I2S_TX_MONO_FST_VLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_update(&self) -> I2S_TX_UPDATE_R {
        I2S_TX_UPDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_tx_big_endian(&self) -> I2S_TX_BIG_ENDIAN_R {
        I2S_TX_BIG_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_tx_chan_equal(&self) -> I2S_TX_CHAN_EQUAL_R {
        I2S_TX_CHAN_EQUAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_tx_mono(&self) -> I2S_TX_MONO_R {
        I2S_TX_MONO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_slave_mod(&self) -> I2S_TX_SLAVE_MOD_R {
        I2S_TX_SLAVE_MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_tx_start(&self) -> I2S_TX_START_R {
        I2S_TX_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn i2s_sig_loopback(&mut self) -> I2S_SIG_LOOPBACK_W {
        I2S_SIG_LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn i2s_tx_chan_mod(&mut self) -> I2S_TX_CHAN_MOD_W {
        I2S_TX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s_tx_pdm_en(&mut self) -> I2S_TX_PDM_EN_W {
        I2S_TX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2s_tx_tdm_en(&mut self) -> I2S_TX_TDM_EN_W {
        I2S_TX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2s_tx_bit_order(&mut self) -> I2S_TX_BIT_ORDER_W {
        I2S_TX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2s_tx_ws_idle_pol(&mut self) -> I2S_TX_WS_IDLE_POL_W {
        I2S_TX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2s_tx_24_fill_en(&mut self) -> I2S_TX_24_FILL_EN_W {
        I2S_TX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2s_tx_left_align(&mut self) -> I2S_TX_LEFT_ALIGN_W {
        I2S_TX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_tx_stop_en(&mut self) -> I2S_TX_STOP_EN_W {
        I2S_TX_STOP_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_tx_pcm_bypass(&mut self) -> I2S_TX_PCM_BYPASS_W {
        I2S_TX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn i2s_tx_pcm_conf(&mut self) -> I2S_TX_PCM_CONF_W {
        I2S_TX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_tx_mono_fst_vld(&mut self) -> I2S_TX_MONO_FST_VLD_W {
        I2S_TX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_tx_update(&mut self) -> I2S_TX_UPDATE_W {
        I2S_TX_UPDATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_tx_big_endian(&mut self) -> I2S_TX_BIG_ENDIAN_W {
        I2S_TX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_tx_chan_equal(&mut self) -> I2S_TX_CHAN_EQUAL_W {
        I2S_TX_CHAN_EQUAL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_tx_mono(&mut self) -> I2S_TX_MONO_W {
        I2S_TX_MONO_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_tx_slave_mod(&mut self) -> I2S_TX_SLAVE_MOD_W {
        I2S_TX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_tx_start(&mut self) -> I2S_TX_START_W {
        I2S_TX_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_tx_fifo_reset(&mut self) -> I2S_TX_FIFO_RESET_W {
        I2S_TX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_tx_reset(&mut self) -> I2S_TX_RESET_W {
        I2S_TX_RESET_W { w: self }
    }
}
