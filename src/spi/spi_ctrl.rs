#[doc = "Reader of register SPI_CTRL"]
pub type R = crate::R<u32, super::SPI_CTRL>;
#[doc = "Writer for register SPI_CTRL"]
pub type W = crate::W<u32, super::SPI_CTRL>;
#[doc = "Register SPI_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_WR_BIT_ORDER`"]
pub type SPI_WR_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WR_BIT_ORDER`"]
pub struct SPI_WR_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WR_BIT_ORDER_W<'a> {
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
#[doc = "Reader of field `SPI_RD_BIT_ORDER`"]
pub type SPI_RD_BIT_ORDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RD_BIT_ORDER`"]
pub struct SPI_RD_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RD_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `SPI_WP_POL`"]
pub type SPI_WP_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_WP_POL`"]
pub struct SPI_WP_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WP_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_HOLD_POL`"]
pub type SPI_HOLD_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_HOLD_POL`"]
pub struct SPI_HOLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_HOLD_POL_W<'a> {
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
#[doc = "Reader of field `SPI_D_POL`"]
pub type SPI_D_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_D_POL`"]
pub struct SPI_D_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_D_POL_W<'a> {
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
#[doc = "Reader of field `SPI_Q_POL`"]
pub type SPI_Q_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_Q_POL`"]
pub struct SPI_Q_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_Q_POL_W<'a> {
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
#[doc = "Reader of field `SPI_FREAD_QUAD`"]
pub type SPI_FREAD_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_QUAD`"]
pub struct SPI_FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_FREAD_DUAL`"]
pub type SPI_FREAD_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FREAD_DUAL`"]
pub struct SPI_FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FREAD_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPI_FCMD_QUAD`"]
pub type SPI_FCMD_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FCMD_QUAD`"]
pub struct SPI_FCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FCMD_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_FCMD_DUAL`"]
pub type SPI_FCMD_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FCMD_DUAL`"]
pub struct SPI_FCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FCMD_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_FADDR_QUAD`"]
pub type SPI_FADDR_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FADDR_QUAD`"]
pub struct SPI_FADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FADDR_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_FADDR_DUAL`"]
pub type SPI_FADDR_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FADDR_DUAL`"]
pub struct SPI_FADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FADDR_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_DUMMY_OUT`"]
pub type SPI_DUMMY_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DUMMY_OUT`"]
pub struct SPI_DUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DUMMY_OUT_W<'a> {
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
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> SPI_WR_BIT_ORDER_R {
        SPI_WR_BIT_ORDER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> SPI_RD_BIT_ORDER_R {
        SPI_RD_BIT_ORDER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_wp_pol(&self) -> SPI_WP_POL_R {
        SPI_WP_POL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_hold_pol(&self) -> SPI_HOLD_POL_R {
        SPI_HOLD_POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_d_pol(&self) -> SPI_D_POL_R {
        SPI_D_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_q_pol(&self) -> SPI_Q_POL_R {
        SPI_Q_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_fread_quad(&self) -> SPI_FREAD_QUAD_R {
        SPI_FREAD_QUAD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_fread_dual(&self) -> SPI_FREAD_DUAL_R {
        SPI_FREAD_DUAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_fcmd_quad(&self) -> SPI_FCMD_QUAD_R {
        SPI_FCMD_QUAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_fcmd_dual(&self) -> SPI_FCMD_DUAL_R {
        SPI_FCMD_DUAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_faddr_quad(&self) -> SPI_FADDR_QUAD_R {
        SPI_FADDR_QUAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_faddr_dual(&self) -> SPI_FADDR_DUAL_R {
        SPI_FADDR_DUAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_dummy_out(&self) -> SPI_DUMMY_OUT_R {
        SPI_DUMMY_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W {
        SPI_WR_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W {
        SPI_RD_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_wp_pol(&mut self) -> SPI_WP_POL_W {
        SPI_WP_POL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_hold_pol(&mut self) -> SPI_HOLD_POL_W {
        SPI_HOLD_POL_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi_d_pol(&mut self) -> SPI_D_POL_W {
        SPI_D_POL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi_q_pol(&mut self) -> SPI_Q_POL_W {
        SPI_Q_POL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_fread_quad(&mut self) -> SPI_FREAD_QUAD_W {
        SPI_FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_fread_dual(&mut self) -> SPI_FREAD_DUAL_W {
        SPI_FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_fcmd_quad(&mut self) -> SPI_FCMD_QUAD_W {
        SPI_FCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_fcmd_dual(&mut self) -> SPI_FCMD_DUAL_W {
        SPI_FCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_faddr_quad(&mut self) -> SPI_FADDR_QUAD_W {
        SPI_FADDR_QUAD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_faddr_dual(&mut self) -> SPI_FADDR_DUAL_W {
        SPI_FADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_dummy_out(&mut self) -> SPI_DUMMY_OUT_W {
        SPI_DUMMY_OUT_W { w: self }
    }
}
