#[doc = "Register `INTR_STATUS_REG_1` reader"]
pub struct R(crate::R<INTR_STATUS_REG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_REG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_REG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_REG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_STATUS_1` reader - Status register for interrupt sources 32~63 mapping register"]
pub type INTR_STATUS_1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Status register for interrupt sources 32~63 mapping register"]
    #[inline(always)]
    pub fn intr_status_1(&self) -> INTR_STATUS_1_R {
        INTR_STATUS_1_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status_reg_1](index.html) module"]
pub struct INTR_STATUS_REG_1_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status_reg_1::R](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_STATUS_REG_1 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
