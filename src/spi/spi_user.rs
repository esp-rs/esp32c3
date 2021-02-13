#[doc = "Reader of register SPI_USER"]
pub type R = crate::R<u32, super::SPI_USER>;
#[doc = "Writer for register SPI_USER"]
pub type W = crate::W<u32, super::SPI_USER>;
#[doc = "Register SPI_USER `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_USER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_USR_COMMAND`"]
pub type SPI_USR_COMMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_COMMAND`"]
pub struct SPI_USR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_COMMAND_W<'a> {
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
#[doc = "Reader of field `SPI_USR_ADDR`"]
pub type SPI_USR_ADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_ADDR`"]
pub struct SPI_USR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_ADDR_W<'a> {
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
#[doc = "Reader of field `SPI_USR_DUMMY`"]
pub type SPI_USR_DUMMY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DUMMY`"]
pub struct SPI_USR_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_W<'a> {
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
#[doc = "Reader of field `SPI_USR_MISO`"]
pub type SPI_USR_MISO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MISO`"]
pub struct SPI_USR_MISO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_USR_MOSI`"]
pub type SPI_USR_MOSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MOSI`"]
pub struct SPI_USR_MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MOSI_W<'a> {
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
#[doc = "Reader of field `SPI_USR_DUMMY_IDLE`"]
pub type SPI_USR_DUMMY_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_DUMMY_IDLE`"]
pub struct SPI_USR_DUMMY_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_DUMMY_IDLE_W<'a> {
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
#[doc = "Reader of field `SPI_USR_MOSI_HIGHPART`"]
pub type SPI_USR_MOSI_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MOSI_HIGHPART`"]
pub struct SPI_USR_MOSI_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MOSI_HIGHPART_W<'a> {
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
#[doc = "Reader of field `SPI_USR_MISO_HIGHPART`"]
pub type SPI_USR_MISO_HIGHPART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_MISO_HIGHPART`"]
pub struct SPI_USR_MISO_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_MISO_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_SIO`"]
pub type SPI_SIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_SIO`"]
pub struct SPI_SIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SIO_W<'a> {
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
#[doc = "Reader of field `SPI_USR_CONF_NXT`"]
pub type SPI_USR_CONF_NXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_USR_CONF_NXT`"]
pub struct SPI_USR_CONF_NXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_USR_CONF_NXT_W<'a> {
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
#[doc = "Reader of field `SPI_FWRITE_QUAD`"]
pub type SPI_FWRITE_QUAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_QUAD`"]
pub struct SPI_FWRITE_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_QUAD_W<'a> {
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
#[doc = "Reader of field `SPI_FWRITE_DUAL`"]
pub type SPI_FWRITE_DUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_FWRITE_DUAL`"]
pub struct SPI_FWRITE_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FWRITE_DUAL_W<'a> {
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
#[doc = "Reader of field `SPI_CK_OUT_EDGE`"]
pub type SPI_CK_OUT_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CK_OUT_EDGE`"]
pub struct SPI_CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_OUT_EDGE_W<'a> {
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
#[doc = "Reader of field `SPI_RSCK_I_EDGE`"]
pub type SPI_RSCK_I_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RSCK_I_EDGE`"]
pub struct SPI_RSCK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RSCK_I_EDGE_W<'a> {
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
#[doc = "Reader of field `SPI_CS_SETUP`"]
pub type SPI_CS_SETUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_SETUP`"]
pub struct SPI_CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_SETUP_W<'a> {
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
#[doc = "Reader of field `SPI_CS_HOLD`"]
pub type SPI_CS_HOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_CS_HOLD`"]
pub struct SPI_CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_HOLD_W<'a> {
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
#[doc = "Reader of field `SPI_TSCK_I_EDGE`"]
pub type SPI_TSCK_I_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_TSCK_I_EDGE`"]
pub struct SPI_TSCK_I_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TSCK_I_EDGE_W<'a> {
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
#[doc = "Reader of field `SPI_QPI_MODE`"]
pub type SPI_QPI_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_QPI_MODE`"]
pub struct SPI_QPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_QPI_MODE_W<'a> {
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
#[doc = "Reader of field `SPI_DOUTDIN`"]
pub type SPI_DOUTDIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DOUTDIN`"]
pub struct SPI_DOUTDIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DOUTDIN_W<'a> {
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
    pub fn spi_usr_command(&self) -> SPI_USR_COMMAND_R {
        SPI_USR_COMMAND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_usr_addr(&self) -> SPI_USR_ADDR_R {
        SPI_USR_ADDR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_usr_dummy(&self) -> SPI_USR_DUMMY_R {
        SPI_USR_DUMMY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_usr_miso(&self) -> SPI_USR_MISO_R {
        SPI_USR_MISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_usr_mosi(&self) -> SPI_USR_MOSI_R {
        SPI_USR_MOSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&self) -> SPI_USR_DUMMY_IDLE_R {
        SPI_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&self) -> SPI_USR_MOSI_HIGHPART_R {
        SPI_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&self) -> SPI_USR_MISO_HIGHPART_R {
        SPI_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_sio(&self) -> SPI_SIO_R {
        SPI_SIO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&self) -> SPI_USR_CONF_NXT_R {
        SPI_USR_CONF_NXT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_fwrite_quad(&self) -> SPI_FWRITE_QUAD_R {
        SPI_FWRITE_QUAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_fwrite_dual(&self) -> SPI_FWRITE_DUAL_R {
        SPI_FWRITE_DUAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_ck_out_edge(&self) -> SPI_CK_OUT_EDGE_R {
        SPI_CK_OUT_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_rsck_i_edge(&self) -> SPI_RSCK_I_EDGE_R {
        SPI_RSCK_I_EDGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_cs_setup(&self) -> SPI_CS_SETUP_R {
        SPI_CS_SETUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_cs_hold(&self) -> SPI_CS_HOLD_R {
        SPI_CS_HOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_tsck_i_edge(&self) -> SPI_TSCK_I_EDGE_R {
        SPI_TSCK_I_EDGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_qpi_mode(&self) -> SPI_QPI_MODE_R {
        SPI_QPI_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_doutdin(&self) -> SPI_DOUTDIN_R {
        SPI_DOUTDIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_usr_command(&mut self) -> SPI_USR_COMMAND_W {
        SPI_USR_COMMAND_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_usr_addr(&mut self) -> SPI_USR_ADDR_W {
        SPI_USR_ADDR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_usr_dummy(&mut self) -> SPI_USR_DUMMY_W {
        SPI_USR_DUMMY_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_usr_miso(&mut self) -> SPI_USR_MISO_W {
        SPI_USR_MISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_usr_mosi(&mut self) -> SPI_USR_MOSI_W {
        SPI_USR_MOSI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_usr_dummy_idle(&mut self) -> SPI_USR_DUMMY_IDLE_W {
        SPI_USR_DUMMY_IDLE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi_usr_mosi_highpart(&mut self) -> SPI_USR_MOSI_HIGHPART_W {
        SPI_USR_MOSI_HIGHPART_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_usr_miso_highpart(&mut self) -> SPI_USR_MISO_HIGHPART_W {
        SPI_USR_MISO_HIGHPART_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_sio(&mut self) -> SPI_SIO_W {
        SPI_SIO_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi_usr_conf_nxt(&mut self) -> SPI_USR_CONF_NXT_W {
        SPI_USR_CONF_NXT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_fwrite_quad(&mut self) -> SPI_FWRITE_QUAD_W {
        SPI_FWRITE_QUAD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_fwrite_dual(&mut self) -> SPI_FWRITE_DUAL_W {
        SPI_FWRITE_DUAL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi_ck_out_edge(&mut self) -> SPI_CK_OUT_EDGE_W {
        SPI_CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_rsck_i_edge(&mut self) -> SPI_RSCK_I_EDGE_W {
        SPI_RSCK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi_cs_setup(&mut self) -> SPI_CS_SETUP_W {
        SPI_CS_SETUP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi_cs_hold(&mut self) -> SPI_CS_HOLD_W {
        SPI_CS_HOLD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi_tsck_i_edge(&mut self) -> SPI_TSCK_I_EDGE_W {
        SPI_TSCK_I_EDGE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_qpi_mode(&mut self) -> SPI_QPI_MODE_W {
        SPI_QPI_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_doutdin(&mut self) -> SPI_DOUTDIN_W {
        SPI_DOUTDIN_W { w: self }
    }
}
