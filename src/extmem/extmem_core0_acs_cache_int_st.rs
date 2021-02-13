#[doc = "Reader of register EXTMEM_CORE0_ACS_CACHE_INT_ST"]
pub type R = crate::R<u32, super::EXTMEM_CORE0_ACS_CACHE_INT_ST>;
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_WR_ICACHE_ST`"]
pub type EXTMEM_CORE0_DBUS_WR_ICACHE_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_REJECT_ST`"]
pub type EXTMEM_CORE0_DBUS_REJECT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_ACS_MSK_ICACHE_ST`"]
pub type EXTMEM_CORE0_DBUS_ACS_MSK_ICACHE_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_REJECT_ST`"]
pub type EXTMEM_CORE0_IBUS_REJECT_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_WR_ICACHE_ST`"]
pub type EXTMEM_CORE0_IBUS_WR_ICACHE_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_IBUS_ACS_MSK_ICACHE_ST`"]
pub type EXTMEM_CORE0_IBUS_ACS_MSK_ICACHE_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn extmem_core0_dbus_wr_icache_st(&self) -> EXTMEM_CORE0_DBUS_WR_ICACHE_ST_R {
        EXTMEM_CORE0_DBUS_WR_ICACHE_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn extmem_core0_dbus_reject_st(&self) -> EXTMEM_CORE0_DBUS_REJECT_ST_R {
        EXTMEM_CORE0_DBUS_REJECT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_core0_dbus_acs_msk_icache_st(&self) -> EXTMEM_CORE0_DBUS_ACS_MSK_ICACHE_ST_R {
        EXTMEM_CORE0_DBUS_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn extmem_core0_ibus_reject_st(&self) -> EXTMEM_CORE0_IBUS_REJECT_ST_R {
        EXTMEM_CORE0_IBUS_REJECT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn extmem_core0_ibus_wr_icache_st(&self) -> EXTMEM_CORE0_IBUS_WR_ICACHE_ST_R {
        EXTMEM_CORE0_IBUS_WR_ICACHE_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn extmem_core0_ibus_acs_msk_icache_st(&self) -> EXTMEM_CORE0_IBUS_ACS_MSK_ICACHE_ST_R {
        EXTMEM_CORE0_IBUS_ACS_MSK_ICACHE_ST_R::new((self.bits & 0x01) != 0)
    }
}
