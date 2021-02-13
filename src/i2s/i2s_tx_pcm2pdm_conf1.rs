#[doc = "Reader of register I2S_TX_PCM2PDM_CONF1"]
pub type R = crate::R<u32, super::I2S_TX_PCM2PDM_CONF1>;
#[doc = "Writer for register I2S_TX_PCM2PDM_CONF1"]
pub type W = crate::W<u32, super::I2S_TX_PCM2PDM_CONF1>;
#[doc = "Register I2S_TX_PCM2PDM_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_TX_PCM2PDM_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_TX_IIR_HP_MULT12_0`"]
pub type I2S_TX_IIR_HP_MULT12_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_IIR_HP_MULT12_0`"]
pub struct I2S_TX_IIR_HP_MULT12_0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_IIR_HP_MULT12_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_IIR_HP_MULT12_5`"]
pub type I2S_TX_IIR_HP_MULT12_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_TX_IIR_HP_MULT12_5`"]
pub struct I2S_TX_IIR_HP_MULT12_5_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_IIR_HP_MULT12_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_FS`"]
pub type I2S_TX_PDM_FS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_TX_PDM_FS`"]
pub struct I2S_TX_PDM_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2S_TX_PDM_FP`"]
pub type I2S_TX_PDM_FP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2S_TX_PDM_FP`"]
pub struct I2S_TX_PDM_FP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TX_PDM_FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn i2s_tx_iir_hp_mult12_0(&self) -> I2S_TX_IIR_HP_MULT12_0_R {
        I2S_TX_IIR_HP_MULT12_0_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn i2s_tx_iir_hp_mult12_5(&self) -> I2S_TX_IIR_HP_MULT12_5_R {
        I2S_TX_IIR_HP_MULT12_5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn i2s_tx_pdm_fs(&self) -> I2S_TX_PDM_FS_R {
        I2S_TX_PDM_FS_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2s_tx_pdm_fp(&self) -> I2S_TX_PDM_FP_R {
        I2S_TX_PDM_FP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn i2s_tx_iir_hp_mult12_0(&mut self) -> I2S_TX_IIR_HP_MULT12_0_W {
        I2S_TX_IIR_HP_MULT12_0_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn i2s_tx_iir_hp_mult12_5(&mut self) -> I2S_TX_IIR_HP_MULT12_5_W {
        I2S_TX_IIR_HP_MULT12_5_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn i2s_tx_pdm_fs(&mut self) -> I2S_TX_PDM_FS_W {
        I2S_TX_PDM_FS_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2s_tx_pdm_fp(&mut self) -> I2S_TX_PDM_FP_W {
        I2S_TX_PDM_FP_W { w: self }
    }
}
