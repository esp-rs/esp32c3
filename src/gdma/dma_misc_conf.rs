#[doc = "Reader of register DMA_MISC_CONF"]
pub type R = crate::R<u32, super::DMA_MISC_CONF>;
#[doc = "Writer for register DMA_MISC_CONF"]
pub type W = crate::W<u32, super::DMA_MISC_CONF>;
#[doc = "Register DMA_MISC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_MISC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_CLK_EN`"]
pub type DMA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_CLK_EN`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `DMA_ARB_PRI_DIS`"]
pub type DMA_ARB_PRI_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_ARB_PRI_DIS`"]
pub struct DMA_ARB_PRI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ARB_PRI_DIS_W<'a> {
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
#[doc = "Reader of field `DMA_AHBM_RST_INTER`"]
pub type DMA_AHBM_RST_INTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_AHBM_RST_INTER`"]
pub struct DMA_AHBM_RST_INTER_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AHBM_RST_INTER_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_arb_pri_dis(&self) -> DMA_ARB_PRI_DIS_R {
        DMA_ARB_PRI_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_ahbm_rst_inter(&self) -> DMA_AHBM_RST_INTER_R {
        DMA_AHBM_RST_INTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_arb_pri_dis(&mut self) -> DMA_ARB_PRI_DIS_W {
        DMA_ARB_PRI_DIS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_ahbm_rst_inter(&mut self) -> DMA_AHBM_RST_INTER_W {
        DMA_AHBM_RST_INTER_W { w: self }
    }
}
