#[doc = "Register `CH%s_DUTY_R` reader"]
pub struct R(crate::R<CH_DUTY_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_DUTY_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_DUTY_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_DUTY_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_CH0_R` reader - This register stores the current duty of output signal on channel %s."]
pub type DUTY_CH0_R_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:18 - This register stores the current duty of output signal on channel %s."]
    #[inline(always)]
    pub fn duty_ch0_r(&self) -> DUTY_CH0_R_R {
        DUTY_CH0_R_R::new(self.bits & 0x0007_ffff)
    }
}
#[doc = "Current duty cycle for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_duty_r](index.html) module"]
pub struct CH_DUTY_R_SPEC;
impl crate::RegisterSpec for CH_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_duty_r::R](R) reader structure"]
impl crate::Readable for CH_DUTY_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%s_DUTY_R to value 0"]
impl crate::Resettable for CH_DUTY_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
