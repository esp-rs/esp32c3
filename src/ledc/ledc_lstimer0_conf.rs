#[doc = "Reader of register LEDC_LSTIMER0_CONF"]
pub type R = crate::R<u32, super::LEDC_LSTIMER0_CONF>;
#[doc = "Writer for register LEDC_LSTIMER0_CONF"]
pub type W = crate::W<u32, super::LEDC_LSTIMER0_CONF>;
#[doc = "Register LEDC_LSTIMER0_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSTIMER0_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LEDC_LSTIMER0_PARA_UP`"]
pub struct LEDC_LSTIMER0_PARA_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_PARA_UP_W<'a> {
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
#[doc = "Reader of field `LEDC_TICK_SEL_LSTIMER0`"]
pub type LEDC_TICK_SEL_LSTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_TICK_SEL_LSTIMER0`"]
pub struct LEDC_TICK_SEL_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TICK_SEL_LSTIMER0_W<'a> {
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
#[doc = "Reader of field `LEDC_LSTIMER0_RST`"]
pub type LEDC_LSTIMER0_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER0_RST`"]
pub struct LEDC_LSTIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_RST_W<'a> {
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
#[doc = "Reader of field `LEDC_LSTIMER0_PAUSE`"]
pub type LEDC_LSTIMER0_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_LSTIMER0_PAUSE`"]
pub struct LEDC_LSTIMER0_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `LEDC_CLK_DIV_LSTIMER0`"]
pub type LEDC_CLK_DIV_LSTIMER0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_CLK_DIV_LSTIMER0`"]
pub struct LEDC_CLK_DIV_LSTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_CLK_DIV_LSTIMER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | (((value as u32) & 0x0003_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEDC_LSTIMER0_DUTY_RES`"]
pub type LEDC_LSTIMER0_DUTY_RES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_LSTIMER0_DUTY_RES`"]
pub struct LEDC_LSTIMER0_DUTY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_LSTIMER0_DUTY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ledc_tick_sel_lstimer0(&self) -> LEDC_TICK_SEL_LSTIMER0_R {
        LEDC_TICK_SEL_LSTIMER0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ledc_lstimer0_rst(&self) -> LEDC_LSTIMER0_RST_R {
        LEDC_LSTIMER0_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ledc_lstimer0_pause(&self) -> LEDC_LSTIMER0_PAUSE_R {
        LEDC_LSTIMER0_PAUSE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 4:21"]
    #[inline(always)]
    pub fn ledc_clk_div_lstimer0(&self) -> LEDC_CLK_DIV_LSTIMER0_R {
        LEDC_CLK_DIV_LSTIMER0_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ledc_lstimer0_duty_res(&self) -> LEDC_LSTIMER0_DUTY_RES_R {
        LEDC_LSTIMER0_DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ledc_lstimer0_para_up(&mut self) -> LEDC_LSTIMER0_PARA_UP_W {
        LEDC_LSTIMER0_PARA_UP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ledc_tick_sel_lstimer0(&mut self) -> LEDC_TICK_SEL_LSTIMER0_W {
        LEDC_TICK_SEL_LSTIMER0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ledc_lstimer0_rst(&mut self) -> LEDC_LSTIMER0_RST_W {
        LEDC_LSTIMER0_RST_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ledc_lstimer0_pause(&mut self) -> LEDC_LSTIMER0_PAUSE_W {
        LEDC_LSTIMER0_PAUSE_W { w: self }
    }
    #[doc = "Bits 4:21"]
    #[inline(always)]
    pub fn ledc_clk_div_lstimer0(&mut self) -> LEDC_CLK_DIV_LSTIMER0_W {
        LEDC_CLK_DIV_LSTIMER0_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ledc_lstimer0_duty_res(&mut self) -> LEDC_LSTIMER0_DUTY_RES_W {
        LEDC_LSTIMER0_DUTY_RES_W { w: self }
    }
}
