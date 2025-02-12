#[doc = "Register `HP_INT_ENA` reader"]
pub struct R(crate::R<HP_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_INT_ENA` writer"]
pub struct W(crate::W<HP_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_INT_ENA_SPEC>;
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
impl From<crate::W<HP_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_EXC_INT_ENA` reader - need_des"]
pub type LP_CPU_EXC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `LP_CPU_EXC_INT_ENA` writer - need_des"]
pub type LP_CPU_EXC_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_INT_ENA_SPEC, bool, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - need_des"]
pub type SDIO_IDLE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - need_des"]
pub type SDIO_IDLE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP_INT_ENA_SPEC, bool, O>;
#[doc = "Field `SW_INT_ENA` reader - need_des"]
pub type SW_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SW_INT_ENA` writer - need_des"]
pub type SW_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HP_INT_ENA_SPEC, bool, O>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_ENA` reader - need_des"]
pub type SOC_SLEEP_REJECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_ENA` writer - need_des"]
pub type SOC_SLEEP_REJECT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_INT_ENA_SPEC, bool, O>;
#[doc = "Field `SOC_WAKEUP_INT_ENA` reader - need_des"]
pub type SOC_WAKEUP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SOC_WAKEUP_INT_ENA` writer - need_des"]
pub type SOC_WAKEUP_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HP_INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_ena(&self) -> LP_CPU_EXC_INT_ENA_R {
        LP_CPU_EXC_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_ena(&self) -> SW_INT_ENA_R {
        SW_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_ena(&self) -> SOC_SLEEP_REJECT_INT_ENA_R {
        SOC_SLEEP_REJECT_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_ena(&self) -> SOC_WAKEUP_INT_ENA_R {
        SOC_WAKEUP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_exc_int_ena(&mut self) -> LP_CPU_EXC_INT_ENA_W<27> {
        LP_CPU_EXC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W<28> {
        SDIO_IDLE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sw_int_ena(&mut self) -> SW_INT_ENA_W<29> {
        SW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_sleep_reject_int_ena(&mut self) -> SOC_SLEEP_REJECT_INT_ENA_W<30> {
        SOC_SLEEP_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_int_ena(&mut self) -> SOC_WAKEUP_INT_ENA_W<31> {
        SOC_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_int_ena](index.html) module"]
pub struct HP_INT_ENA_SPEC;
impl crate::RegisterSpec for HP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_int_ena::R](R) reader structure"]
impl crate::Readable for HP_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_int_ena::W](W) writer structure"]
impl crate::Writable for HP_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_INT_ENA to value 0"]
impl crate::Resettable for HP_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
