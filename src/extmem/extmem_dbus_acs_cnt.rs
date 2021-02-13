#[doc = "Reader of register EXTMEM_DBUS_ACS_CNT"]
pub type R = crate::R<u32, super::EXTMEM_DBUS_ACS_CNT>;
#[doc = "Reader of field `EXTMEM_DBUS_ACS_CNT`"]
pub type EXTMEM_DBUS_ACS_CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn extmem_dbus_acs_cnt(&self) -> EXTMEM_DBUS_ACS_CNT_R {
        EXTMEM_DBUS_ACS_CNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
