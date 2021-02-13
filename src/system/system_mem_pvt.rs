#[doc = "Reader of register SYSTEM_MEM_PVT"]
pub type R = crate::R<u32, super::SYSTEM_MEM_PVT>;
#[doc = "Writer for register SYSTEM_MEM_PVT"]
pub type W = crate::W<u32, super::SYSTEM_MEM_PVT>;
#[doc = "Register SYSTEM_MEM_PVT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSTEM_MEM_PVT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSTEM_MEM_VT_SEL`"]
pub type SYSTEM_MEM_VT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_MEM_VT_SEL`"]
pub struct SYSTEM_MEM_VT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_MEM_VT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `SYSTEM_MEM_TIMING_ERR_CNT`"]
pub type SYSTEM_MEM_TIMING_ERR_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `SYSTEM_MEM_PVT_MONITOR_EN`"]
pub type SYSTEM_MEM_PVT_MONITOR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSTEM_MEM_PVT_MONITOR_EN`"]
pub struct SYSTEM_MEM_PVT_MONITOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_MEM_PVT_MONITOR_EN_W<'a> {
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
#[doc = "Write proxy for field `SYSTEM_MEM_ERR_CNT_CLR`"]
pub struct SYSTEM_MEM_ERR_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_MEM_ERR_CNT_CLR_W<'a> {
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
#[doc = "Reader of field `SYSTEM_MEM_PATH_LEN`"]
pub type SYSTEM_MEM_PATH_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSTEM_MEM_PATH_LEN`"]
pub struct SYSTEM_MEM_PATH_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_MEM_PATH_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn system_mem_vt_sel(&self) -> SYSTEM_MEM_VT_SEL_R {
        SYSTEM_MEM_VT_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn system_mem_timing_err_cnt(&self) -> SYSTEM_MEM_TIMING_ERR_CNT_R {
        SYSTEM_MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_mem_pvt_monitor_en(&self) -> SYSTEM_MEM_PVT_MONITOR_EN_R {
        SYSTEM_MEM_PVT_MONITOR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn system_mem_path_len(&self) -> SYSTEM_MEM_PATH_LEN_R {
        SYSTEM_MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn system_mem_vt_sel(&mut self) -> SYSTEM_MEM_VT_SEL_W {
        SYSTEM_MEM_VT_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn system_mem_pvt_monitor_en(&mut self) -> SYSTEM_MEM_PVT_MONITOR_EN_W {
        SYSTEM_MEM_PVT_MONITOR_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn system_mem_err_cnt_clr(&mut self) -> SYSTEM_MEM_ERR_CNT_CLR_W {
        SYSTEM_MEM_ERR_CNT_CLR_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn system_mem_path_len(&mut self) -> SYSTEM_MEM_PATH_LEN_W {
        SYSTEM_MEM_PATH_LEN_W { w: self }
    }
}
