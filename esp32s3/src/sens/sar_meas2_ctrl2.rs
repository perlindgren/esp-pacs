#[doc = "Register `SAR_MEAS2_CTRL2` reader"]
pub struct R(crate::R<SAR_MEAS2_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_CTRL2` writer"]
pub struct W(crate::W<SAR_MEAS2_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAR_MEAS2_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEAS2_DATA_SAR` reader - SAR ADC2 data"]
pub type MEAS2_DATA_SAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MEAS2_DONE_SAR` reader - SAR ADC2 conversion done indication"]
pub type MEAS2_DONE_SAR_R = crate::BitReader<bool>;
#[doc = "Field `MEAS2_START_SAR` reader - SAR ADC2 controller (in RTC) starts conversion"]
pub type MEAS2_START_SAR_R = crate::BitReader<bool>;
#[doc = "Field `MEAS2_START_SAR` writer - SAR ADC2 controller (in RTC) starts conversion"]
pub type MEAS2_START_SAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS2_CTRL2_SPEC, bool, O>;
#[doc = "Field `MEAS2_START_FORCE` reader - 1: SAR ADC2 controller (in RTC) is started by SW"]
pub type MEAS2_START_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `MEAS2_START_FORCE` writer - 1: SAR ADC2 controller (in RTC) is started by SW"]
pub type MEAS2_START_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS2_CTRL2_SPEC, bool, O>;
#[doc = "Field `SAR2_EN_PAD` reader - SAR ADC2 pad enable bitmap"]
pub type SAR2_EN_PAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR2_EN_PAD` writer - SAR ADC2 pad enable bitmap"]
pub type SAR2_EN_PAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_MEAS2_CTRL2_SPEC, u16, u16, 12, O>;
#[doc = "Field `SAR2_EN_PAD_FORCE` reader - 1: SAR ADC2 pad enable bitmap is controlled by SW"]
pub type SAR2_EN_PAD_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_EN_PAD_FORCE` writer - 1: SAR ADC2 pad enable bitmap is controlled by SW"]
pub type SAR2_EN_PAD_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_MEAS2_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - SAR ADC2 data"]
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> MEAS2_DATA_SAR_R {
        MEAS2_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - SAR ADC2 conversion done indication"]
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> MEAS2_DONE_SAR_R {
        MEAS2_DONE_SAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion"]
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> MEAS2_START_SAR_R {
        MEAS2_START_SAR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW"]
    #[inline(always)]
    pub fn meas2_start_force(&self) -> MEAS2_START_FORCE_R {
        MEAS2_START_FORCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap"]
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> SAR2_EN_PAD_R {
        SAR2_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW"]
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> SAR2_EN_PAD_FORCE_R {
        SAR2_EN_PAD_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion"]
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_sar(&mut self) -> MEAS2_START_SAR_W<17> {
        MEAS2_START_SAR_W::new(self)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW"]
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_force(&mut self) -> MEAS2_START_FORCE_W<18> {
        MEAS2_START_FORCE_W::new(self)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad(&mut self) -> SAR2_EN_PAD_W<19> {
        SAR2_EN_PAD_W::new(self)
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad_force(&mut self) -> SAR2_EN_PAD_FORCE_W<31> {
        SAR2_EN_PAD_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure saradc2 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_ctrl2](index.html) module"]
pub struct SAR_MEAS2_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_MEAS2_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_MEAS2_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_CTRL2 to value 0"]
impl crate::Resettable for SAR_MEAS2_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
