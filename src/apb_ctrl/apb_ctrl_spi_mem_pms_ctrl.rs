#[doc = "Reader of register APB_CTRL_SPI_MEM_PMS_CTRL"]
pub type R = crate::R<u32, super::APB_CTRL_SPI_MEM_PMS_CTRL>;
#[doc = "Writer for register APB_CTRL_SPI_MEM_PMS_CTRL"]
pub type W = crate::W<u32, super::APB_CTRL_SPI_MEM_PMS_CTRL>;
#[doc = "Register APB_CTRL_SPI_MEM_PMS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_SPI_MEM_PMS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_SPI_MEM_REJECT_CDE`"]
pub type APB_CTRL_SPI_MEM_REJECT_CDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB_CTRL_SPI_MEM_REJECT_CLR`"]
pub struct APB_CTRL_SPI_MEM_REJECT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_SPI_MEM_REJECT_CLR_W<'a> {
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
#[doc = "Reader of field `APB_CTRL_SPI_MEM_REJECT_INT`"]
pub type APB_CTRL_SPI_MEM_REJECT_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn apb_ctrl_spi_mem_reject_cde(&self) -> APB_CTRL_SPI_MEM_REJECT_CDE_R {
        APB_CTRL_SPI_MEM_REJECT_CDE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_spi_mem_reject_int(&self) -> APB_CTRL_SPI_MEM_REJECT_INT_R {
        APB_CTRL_SPI_MEM_REJECT_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb_ctrl_spi_mem_reject_clr(&mut self) -> APB_CTRL_SPI_MEM_REJECT_CLR_W {
        APB_CTRL_SPI_MEM_REJECT_CLR_W { w: self }
    }
}
