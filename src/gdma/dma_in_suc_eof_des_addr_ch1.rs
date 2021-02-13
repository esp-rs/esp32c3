#[doc = "Reader of register DMA_IN_SUC_EOF_DES_ADDR_CH1"]
pub type R = crate::R<u32, super::DMA_IN_SUC_EOF_DES_ADDR_CH1>;
#[doc = "Reader of field `DMA_IN_SUC_EOF_DES_ADDR_CH1`"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_in_suc_eof_des_addr_ch1(&self) -> DMA_IN_SUC_EOF_DES_ADDR_CH1_R {
        DMA_IN_SUC_EOF_DES_ADDR_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
