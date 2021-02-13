#[doc = "Reader of register DMA_IN_ERR_EOF_DES_ADDR_CH2"]
pub type R = crate::R<u32, super::DMA_IN_ERR_EOF_DES_ADDR_CH2>;
#[doc = "Reader of field `DMA_IN_ERR_EOF_DES_ADDR_CH2`"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_in_err_eof_des_addr_ch2(&self) -> DMA_IN_ERR_EOF_DES_ADDR_CH2_R {
        DMA_IN_ERR_EOF_DES_ADDR_CH2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
