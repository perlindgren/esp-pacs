#[doc = "Register `WDTCONFIG0` reader"]
pub struct R(crate::R<WDTCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG0` writer"]
pub struct W(crate::W<WDTCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG0_SPEC>;
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
impl From<crate::W<WDTCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_PAUSE_IN_SLP` reader - need_des"]
pub type WDT_PAUSE_IN_SLP_R = crate::BitReader<bool>;
#[doc = "Field `WDT_PAUSE_IN_SLP` writer - need_des"]
pub type WDT_PAUSE_IN_SLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - need_des"]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - need_des"]
pub type WDT_APPCPU_RESET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - need_des"]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - need_des"]
pub type WDT_PROCPU_RESET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - need_des"]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - need_des"]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - need_des"]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - need_des"]
pub type WDT_SYS_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - need_des"]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - need_des"]
pub type WDT_CPU_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_STG3` reader - need_des"]
pub type WDT_STG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG3` writer - need_des"]
pub type WDT_STG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_STG2` reader - need_des"]
pub type WDT_STG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG2` writer - need_des"]
pub type WDT_STG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_STG1` reader - need_des"]
pub type WDT_STG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG1` writer - need_des"]
pub type WDT_STG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_STG0` reader - need_des"]
pub type WDT_STG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_STG0` writer - need_des"]
pub type WDT_STG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTCONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WDT_EN` reader - need_des"]
pub type WDT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EN` writer - need_des"]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDTCONFIG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> WDT_PAUSE_IN_SLP_R {
        WDT_PAUSE_IN_SLP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause_in_slp(&mut self) -> WDT_PAUSE_IN_SLP_W<9> {
        WDT_PAUSE_IN_SLP_W::new(self)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<10> {
        WDT_APPCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<11> {
        WDT_PROCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<12> {
        WDT_FLASHBOOT_MOD_EN_W::new(self)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<13> {
        WDT_SYS_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<16> {
        WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<19> {
        WDT_STG3_W::new(self)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<22> {
        WDT_STG2_W::new(self)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<25> {
        WDT_STG1_W::new(self)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<28> {
        WDT_STG0_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<31> {
        WDT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig0::R](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0001_3200"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_3200;
}
