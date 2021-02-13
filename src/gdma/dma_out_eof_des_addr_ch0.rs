#[doc = "Reader of register DMA_OUT_EOF_DES_ADDR_CH0"]
pub type R = crate::R<u32, super::DMA_OUT_EOF_DES_ADDR_CH0>;
#[doc = "Reader of field `DMA_OUT_EOF_DES_ADDR_CH0`"]
pub type DMA_OUT_EOF_DES_ADDR_CH0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_out_eof_des_addr_ch0(&self) -> DMA_OUT_EOF_DES_ADDR_CH0_R {
        DMA_OUT_EOF_DES_ADDR_CH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
