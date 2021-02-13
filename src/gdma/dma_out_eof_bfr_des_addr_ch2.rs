#[doc = "Reader of register DMA_OUT_EOF_BFR_DES_ADDR_CH2"]
pub type R = crate::R<u32, super::DMA_OUT_EOF_BFR_DES_ADDR_CH2>;
#[doc = "Reader of field `DMA_OUT_EOF_BFR_DES_ADDR_CH2`"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_out_eof_bfr_des_addr_ch2(&self) -> DMA_OUT_EOF_BFR_DES_ADDR_CH2_R {
        DMA_OUT_EOF_BFR_DES_ADDR_CH2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
