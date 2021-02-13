#[doc = "Reader of register RMT_SYS_CONF"]
pub type R = crate::R<u32, super::RMT_SYS_CONF>;
#[doc = "Writer for register RMT_SYS_CONF"]
pub type W = crate::W<u32, super::RMT_SYS_CONF>;
#[doc = "Register RMT_SYS_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_SYS_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CLK_EN`"]
pub type RMT_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CLK_EN`"]
pub struct RMT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RMT_SCLK_ACTIVE`"]
pub type RMT_SCLK_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_SCLK_ACTIVE`"]
pub struct RMT_SCLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_SCLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RMT_SCLK_SEL`"]
pub type RMT_SCLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_SCLK_SEL`"]
pub struct RMT_SCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_SCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMT_SCLK_DIV_B`"]
pub type RMT_SCLK_DIV_B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_SCLK_DIV_B`"]
pub struct RMT_SCLK_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_SCLK_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RMT_SCLK_DIV_A`"]
pub type RMT_SCLK_DIV_A_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_SCLK_DIV_A`"]
pub struct RMT_SCLK_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_SCLK_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RMT_SCLK_DIV_NUM`"]
pub type RMT_SCLK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_SCLK_DIV_NUM`"]
pub struct RMT_SCLK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_SCLK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_FORCE_PU`"]
pub type RMT_MEM_FORCE_PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_FORCE_PU`"]
pub struct RMT_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_FORCE_PU_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_FORCE_PD`"]
pub type RMT_MEM_FORCE_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_FORCE_PD`"]
pub struct RMT_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_FORCE_PD_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_CLK_FORCE_ON`"]
pub type RMT_MEM_CLK_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_CLK_FORCE_ON`"]
pub struct RMT_MEM_CLK_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_CLK_FORCE_ON_W<'a> {
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
#[doc = "Reader of field `RMT_APB_FIFO_MASK`"]
pub type RMT_APB_FIFO_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_APB_FIFO_MASK`"]
pub struct RMT_APB_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_FIFO_MASK_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rmt_sclk_active(&self) -> RMT_SCLK_ACTIVE_R {
        RMT_SCLK_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rmt_sclk_sel(&self) -> RMT_SCLK_SEL_R {
        RMT_SCLK_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rmt_sclk_div_b(&self) -> RMT_SCLK_DIV_B_R {
        RMT_SCLK_DIV_B_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn rmt_sclk_div_a(&self) -> RMT_SCLK_DIV_A_R {
        RMT_SCLK_DIV_A_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn rmt_sclk_div_num(&self) -> RMT_SCLK_DIV_NUM_R {
        RMT_SCLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_mem_force_pu(&self) -> RMT_MEM_FORCE_PU_R {
        RMT_MEM_FORCE_PU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_mem_force_pd(&self) -> RMT_MEM_FORCE_PD_R {
        RMT_MEM_FORCE_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&self) -> RMT_MEM_CLK_FORCE_ON_R {
        RMT_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_apb_fifo_mask(&self) -> RMT_APB_FIFO_MASK_R {
        RMT_APB_FIFO_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W {
        RMT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rmt_sclk_active(&mut self) -> RMT_SCLK_ACTIVE_W {
        RMT_SCLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rmt_sclk_sel(&mut self) -> RMT_SCLK_SEL_W {
        RMT_SCLK_SEL_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rmt_sclk_div_b(&mut self) -> RMT_SCLK_DIV_B_W {
        RMT_SCLK_DIV_B_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn rmt_sclk_div_a(&mut self) -> RMT_SCLK_DIV_A_W {
        RMT_SCLK_DIV_A_W { w: self }
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn rmt_sclk_div_num(&mut self) -> RMT_SCLK_DIV_NUM_W {
        RMT_SCLK_DIV_NUM_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rmt_mem_force_pu(&mut self) -> RMT_MEM_FORCE_PU_W {
        RMT_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rmt_mem_force_pd(&mut self) -> RMT_MEM_FORCE_PD_W {
        RMT_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&mut self) -> RMT_MEM_CLK_FORCE_ON_W {
        RMT_MEM_CLK_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmt_apb_fifo_mask(&mut self) -> RMT_APB_FIFO_MASK_W {
        RMT_APB_FIFO_MASK_W { w: self }
    }
}
