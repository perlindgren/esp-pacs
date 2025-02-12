#[doc = "Register `PKT_LEN` reader"]
pub struct R(crate::R<PKT_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKT_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKT_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKT_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOSTSLCHOST_SLC0_LEN` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOSTSLCHOST_SLC0_LEN_CHECK` reader - *******Description***********"]
pub type HOSTSLCHOST_SLC0_LEN_CHECK_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:19 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len(&self) -> HOSTSLCHOST_SLC0_LEN_R {
        HOSTSLCHOST_SLC0_LEN_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - *******Description***********"]
    #[inline(always)]
    pub fn hostslchost_slc0_len_check(&self) -> HOSTSLCHOST_SLC0_LEN_CHECK_R {
        HOSTSLCHOST_SLC0_LEN_CHECK_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkt_len](index.html) module"]
pub struct PKT_LEN_SPEC;
impl crate::RegisterSpec for PKT_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkt_len::R](R) reader structure"]
impl crate::Readable for PKT_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKT_LEN to value 0"]
impl crate::Resettable for PKT_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
