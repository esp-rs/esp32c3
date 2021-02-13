#[doc = "Reader of register SPI_MISC"]
pub type R = crate::R<u32, super::SPI_MISC>;
#[doc = "Writer for register SPI_MISC"]
pub type W = crate::W<u32, super::SPI_MISC>;
#[doc = "Register SPI_MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_QUAD_DIN_PIN_SWAP`"]
pub type SPI_QUAD_DIN_PIN_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_QUAD_DIN_PIN_SWAP`"]
pub struct SPI_QUAD_DIN_PIN_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_QUAD_DIN_PIN_SWAP_W<'a> {
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
#[doc = "Reader of field `SPI_CS_KEEP_ACTIVE`"]
pub type SPI_CS_KEEP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_KEEP_ACTIVE`"]
pub struct SPI_CS_KEEP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_KEEP_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_IDLE_EDGE`"]
pub type SPI_CK_IDLE_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_IDLE_EDGE`"]
pub struct SPI_CK_IDLE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_IDLE_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SPI_SLAVE_CS_POL`"]
pub type SPI_SLAVE_CS_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SLAVE_CS_POL`"]
pub struct SPI_SLAVE_CS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLAVE_CS_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPI_MASTER_CS_POL`"]
pub type SPI_MASTER_CS_POL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MASTER_CS_POL`"]
pub struct SPI_MASTER_CS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MASTER_CS_POL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_DIS`"]
pub type SPI_CK_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_DIS`"]
pub struct SPI_CK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS5_DIS`"]
pub type SPI_CS5_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS5_DIS`"]
pub struct SPI_CS5_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS5_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS4_DIS`"]
pub type SPI_CS4_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS4_DIS`"]
pub struct SPI_CS4_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS4_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS3_DIS`"]
pub type SPI_CS3_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS3_DIS`"]
pub struct SPI_CS3_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS3_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS2_DIS`"]
pub type SPI_CS2_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS2_DIS`"]
pub struct SPI_CS2_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS2_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS1_DIS`"]
pub type SPI_CS1_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS1_DIS`"]
pub struct SPI_CS1_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS1_DIS_W<'a> {
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
#[doc = "Reader of field `SPI_CS0_DIS`"]
pub type SPI_CS0_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS0_DIS`"]
pub struct SPI_CS0_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS0_DIS_W<'a> {
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
    pub fn spi_quad_din_pin_swap(&self) -> SPI_QUAD_DIN_PIN_SWAP_R {
        SPI_QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_cs_keep_active(&self) -> SPI_CS_KEEP_ACTIVE_R {
        SPI_CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> SPI_CK_IDLE_EDGE_R {
        SPI_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&self) -> SPI_SLAVE_CS_POL_R {
        SPI_SLAVE_CS_POL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn spi_master_cs_pol(&self) -> SPI_MASTER_CS_POL_R {
        SPI_MASTER_CS_POL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> SPI_CK_DIS_R {
        SPI_CK_DIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_cs5_dis(&self) -> SPI_CS5_DIS_R {
        SPI_CS5_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_cs4_dis(&self) -> SPI_CS4_DIS_R {
        SPI_CS4_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_cs3_dis(&self) -> SPI_CS3_DIS_R {
        SPI_CS3_DIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&mut self) -> SPI_QUAD_DIN_PIN_SWAP_W {
        SPI_QUAD_DIN_PIN_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_cs_keep_active(&mut self) -> SPI_CS_KEEP_ACTIVE_W {
        SPI_CS_KEEP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&mut self) -> SPI_CK_IDLE_EDGE_W {
        SPI_CK_IDLE_EDGE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&mut self) -> SPI_SLAVE_CS_POL_W {
        SPI_SLAVE_CS_POL_W { w: self }
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn spi_master_cs_pol(&mut self) -> SPI_MASTER_CS_POL_W {
        SPI_MASTER_CS_POL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_ck_dis(&mut self) -> SPI_CK_DIS_W {
        SPI_CK_DIS_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_cs5_dis(&mut self) -> SPI_CS5_DIS_W {
        SPI_CS5_DIS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_cs4_dis(&mut self) -> SPI_CS4_DIS_W {
        SPI_CS4_DIS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_cs3_dis(&mut self) -> SPI_CS3_DIS_W {
        SPI_CS3_DIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W {
        SPI_CS2_DIS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W {
        SPI_CS1_DIS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W {
        SPI_CS0_DIS_W { w: self }
    }
}
