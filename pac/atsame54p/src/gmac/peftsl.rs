#[doc = "Register `PEFTSL` reader"]
pub struct R(crate::R<PEFTSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEFTSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEFTSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEFTSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUD` reader - Register Update"]
pub struct RUD_R(crate::FieldReader<u32, u32>);
impl RUD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Register Update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(self.bits as u32)
    }
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peftsl](index.html) module"]
pub struct PEFTSL_SPEC;
impl crate::RegisterSpec for PEFTSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peftsl::R](R) reader structure"]
impl crate::Readable for PEFTSL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEFTSL to value 0"]
impl crate::Resettable for PEFTSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
