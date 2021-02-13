#[doc = "Reader of register EFUSE_RD_RS_ERR0"]
pub type R = crate::R<u32, super::EFUSE_RD_RS_ERR0>;
#[doc = "Reader of field `EFUSE_KEY4_FAIL`"]
pub type EFUSE_KEY4_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY4_ERR_NUM`"]
pub type EFUSE_KEY4_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY3_FAIL`"]
pub type EFUSE_KEY3_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY3_ERR_NUM`"]
pub type EFUSE_KEY3_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY2_FAIL`"]
pub type EFUSE_KEY2_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY2_ERR_NUM`"]
pub type EFUSE_KEY2_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY1_FAIL`"]
pub type EFUSE_KEY1_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY1_ERR_NUM`"]
pub type EFUSE_KEY1_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_KEY0_FAIL`"]
pub type EFUSE_KEY0_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_KEY0_ERR_NUM`"]
pub type EFUSE_KEY0_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_USR_DATA_FAIL`"]
pub type EFUSE_USR_DATA_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_USR_DATA_ERR_NUM`"]
pub type EFUSE_USR_DATA_ERR_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SYS_PART1_FAIL`"]
pub type EFUSE_SYS_PART1_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_SYS_PART1_NUM`"]
pub type EFUSE_SYS_PART1_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_MAC_SPI_8M_FAIL`"]
pub type EFUSE_MAC_SPI_8M_FAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_MAC_SPI_8M_ERR_NUM`"]
pub type EFUSE_MAC_SPI_8M_ERR_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn efuse_key4_fail(&self) -> EFUSE_KEY4_FAIL_R {
        EFUSE_KEY4_FAIL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn efuse_key4_err_num(&self) -> EFUSE_KEY4_ERR_NUM_R {
        EFUSE_KEY4_ERR_NUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn efuse_key3_fail(&self) -> EFUSE_KEY3_FAIL_R {
        EFUSE_KEY3_FAIL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn efuse_key3_err_num(&self) -> EFUSE_KEY3_ERR_NUM_R {
        EFUSE_KEY3_ERR_NUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn efuse_key2_fail(&self) -> EFUSE_KEY2_FAIL_R {
        EFUSE_KEY2_FAIL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn efuse_key2_err_num(&self) -> EFUSE_KEY2_ERR_NUM_R {
        EFUSE_KEY2_ERR_NUM_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn efuse_key1_fail(&self) -> EFUSE_KEY1_FAIL_R {
        EFUSE_KEY1_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn efuse_key1_err_num(&self) -> EFUSE_KEY1_ERR_NUM_R {
        EFUSE_KEY1_ERR_NUM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn efuse_key0_fail(&self) -> EFUSE_KEY0_FAIL_R {
        EFUSE_KEY0_FAIL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn efuse_key0_err_num(&self) -> EFUSE_KEY0_ERR_NUM_R {
        EFUSE_KEY0_ERR_NUM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn efuse_usr_data_fail(&self) -> EFUSE_USR_DATA_FAIL_R {
        EFUSE_USR_DATA_FAIL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn efuse_usr_data_err_num(&self) -> EFUSE_USR_DATA_ERR_NUM_R {
        EFUSE_USR_DATA_ERR_NUM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn efuse_sys_part1_fail(&self) -> EFUSE_SYS_PART1_FAIL_R {
        EFUSE_SYS_PART1_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn efuse_sys_part1_num(&self) -> EFUSE_SYS_PART1_NUM_R {
        EFUSE_SYS_PART1_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_mac_spi_8m_fail(&self) -> EFUSE_MAC_SPI_8M_FAIL_R {
        EFUSE_MAC_SPI_8M_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn efuse_mac_spi_8m_err_num(&self) -> EFUSE_MAC_SPI_8M_ERR_NUM_R {
        EFUSE_MAC_SPI_8M_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
}
