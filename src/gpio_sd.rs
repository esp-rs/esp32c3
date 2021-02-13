#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_SIGMADELTA0"]
    pub gpio_sigmadelta0: GPIO_SIGMADELTA0,
    #[doc = "0x04 - GPIO_SIGMADELTA1"]
    pub gpio_sigmadelta1: GPIO_SIGMADELTA1,
    #[doc = "0x08 - GPIO_SIGMADELTA2"]
    pub gpio_sigmadelta2: GPIO_SIGMADELTA2,
    #[doc = "0x0c - GPIO_SIGMADELTA3"]
    pub gpio_sigmadelta3: GPIO_SIGMADELTA3,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - GPIO_SIGMADELTA_CG"]
    pub gpio_sigmadelta_cg: GPIO_SIGMADELTA_CG,
    #[doc = "0x24 - GPIO_SIGMADELTA_MISC"]
    pub gpio_sigmadelta_misc: GPIO_SIGMADELTA_MISC,
    #[doc = "0x28 - GPIO_SIGMADELTA_VERSION"]
    pub gpio_sigmadelta_version: GPIO_SIGMADELTA_VERSION,
}
#[doc = "GPIO_SIGMADELTA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta0](gpio_sigmadelta0) module"]
pub type GPIO_SIGMADELTA0 = crate::Reg<u32, _GPIO_SIGMADELTA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA0;
#[doc = "`read()` method returns [gpio_sigmadelta0::R](gpio_sigmadelta0::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA0 {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta0::W](gpio_sigmadelta0::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA0 {}
#[doc = "GPIO_SIGMADELTA0"]
pub mod gpio_sigmadelta0;
#[doc = "GPIO_SIGMADELTA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta1](gpio_sigmadelta1) module"]
pub type GPIO_SIGMADELTA1 = crate::Reg<u32, _GPIO_SIGMADELTA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA1;
#[doc = "`read()` method returns [gpio_sigmadelta1::R](gpio_sigmadelta1::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA1 {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta1::W](gpio_sigmadelta1::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA1 {}
#[doc = "GPIO_SIGMADELTA1"]
pub mod gpio_sigmadelta1;
#[doc = "GPIO_SIGMADELTA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta2](gpio_sigmadelta2) module"]
pub type GPIO_SIGMADELTA2 = crate::Reg<u32, _GPIO_SIGMADELTA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA2;
#[doc = "`read()` method returns [gpio_sigmadelta2::R](gpio_sigmadelta2::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA2 {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta2::W](gpio_sigmadelta2::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA2 {}
#[doc = "GPIO_SIGMADELTA2"]
pub mod gpio_sigmadelta2;
#[doc = "GPIO_SIGMADELTA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta3](gpio_sigmadelta3) module"]
pub type GPIO_SIGMADELTA3 = crate::Reg<u32, _GPIO_SIGMADELTA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA3;
#[doc = "`read()` method returns [gpio_sigmadelta3::R](gpio_sigmadelta3::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA3 {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta3::W](gpio_sigmadelta3::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA3 {}
#[doc = "GPIO_SIGMADELTA3"]
pub mod gpio_sigmadelta3;
#[doc = "GPIO_SIGMADELTA_CG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta_cg](gpio_sigmadelta_cg) module"]
pub type GPIO_SIGMADELTA_CG = crate::Reg<u32, _GPIO_SIGMADELTA_CG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_CG;
#[doc = "`read()` method returns [gpio_sigmadelta_cg::R](gpio_sigmadelta_cg::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_CG {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_cg::W](gpio_sigmadelta_cg::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_CG {}
#[doc = "GPIO_SIGMADELTA_CG"]
pub mod gpio_sigmadelta_cg;
#[doc = "GPIO_SIGMADELTA_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta_misc](gpio_sigmadelta_misc) module"]
pub type GPIO_SIGMADELTA_MISC = crate::Reg<u32, _GPIO_SIGMADELTA_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_MISC;
#[doc = "`read()` method returns [gpio_sigmadelta_misc::R](gpio_sigmadelta_misc::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_MISC {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_misc::W](gpio_sigmadelta_misc::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_MISC {}
#[doc = "GPIO_SIGMADELTA_MISC"]
pub mod gpio_sigmadelta_misc;
#[doc = "GPIO_SIGMADELTA_VERSION\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_sigmadelta_version](gpio_sigmadelta_version) module"]
pub type GPIO_SIGMADELTA_VERSION = crate::Reg<u32, _GPIO_SIGMADELTA_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_SIGMADELTA_VERSION;
#[doc = "`read()` method returns [gpio_sigmadelta_version::R](gpio_sigmadelta_version::R) reader structure"]
impl crate::Readable for GPIO_SIGMADELTA_VERSION {}
#[doc = "`write(|w| ..)` method takes [gpio_sigmadelta_version::W](gpio_sigmadelta_version::W) writer structure"]
impl crate::Writable for GPIO_SIGMADELTA_VERSION {}
#[doc = "GPIO_SIGMADELTA_VERSION"]
pub mod gpio_sigmadelta_version;
