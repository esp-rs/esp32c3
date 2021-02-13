#[doc = "Reader of register I2C_FIFO_ST"]
pub type R = crate::R<u32, super::I2C_FIFO_ST>;
#[doc = "Reader of field `I2C_SLAVE_RW_POINT`"]
pub type I2C_SLAVE_RW_POINT_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_TXFIFO_WADDR`"]
pub type I2C_TXFIFO_WADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_TXFIFO_RADDR`"]
pub type I2C_TXFIFO_RADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_RXFIFO_WADDR`"]
pub type I2C_RXFIFO_WADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `I2C_RXFIFO_RADDR`"]
pub type I2C_RXFIFO_RADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn i2c_slave_rw_point(&self) -> I2C_SLAVE_RW_POINT_R {
        I2C_SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn i2c_txfifo_waddr(&self) -> I2C_TXFIFO_WADDR_R {
        I2C_TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn i2c_txfifo_raddr(&self) -> I2C_TXFIFO_RADDR_R {
        I2C_TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn i2c_rxfifo_waddr(&self) -> I2C_RXFIFO_WADDR_R {
        I2C_RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2c_rxfifo_raddr(&self) -> I2C_RXFIFO_RADDR_R {
        I2C_RXFIFO_RADDR_R::new((self.bits & 0x1f) as u8)
    }
}
