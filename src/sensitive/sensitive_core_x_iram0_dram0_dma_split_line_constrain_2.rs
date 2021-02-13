#[doc = "Reader of register SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2"]
pub type R = crate::R<u32, super::SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2>;
#[doc = "Writer for register SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2"]
pub type W = crate::W<u32, super::SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2>;
#[doc = "Register SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR`"]
pub type SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR`"]
pub struct SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | (((value as u32) & 0xff) << 14);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2`"]
pub type SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2`"]
pub struct SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1`"]
pub type SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1`"]
pub struct SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0`"]
pub type SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0`"]
pub struct SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_splitaddr(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_R {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_2(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_R {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_1(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_R {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_0(
        &self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_R {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_splitaddr(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_W {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_SPLITADDR_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_2(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_W {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_2_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_1(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_W {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sensitive_core_x_iram0_sram_line_0_category_0(
        &mut self,
    ) -> SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_W {
        SENSITIVE_CORE_X_IRAM0_SRAM_LINE_0_CATEGORY_0_W { w: self }
    }
}
