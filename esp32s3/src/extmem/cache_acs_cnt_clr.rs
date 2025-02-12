#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub struct W(crate::W<CACHE_ACS_CNT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
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
impl From<crate::W<CACHE_ACS_CNT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ACS_CNT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_ACS_CNT_CLR` writer - The bit is used to clear dcache counter."]
pub type DCACHE_ACS_CNT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ACS_CNT_CLR_SPEC, bool, O>;
#[doc = "Field `ICACHE_ACS_CNT_CLR` writer - The bit is used to clear icache counter."]
pub type ICACHE_ACS_CNT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_ACS_CNT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - The bit is used to clear dcache counter."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_acs_cnt_clr(&mut self) -> DCACHE_ACS_CNT_CLR_W<0> {
        DCACHE_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear icache counter."]
    #[inline(always)]
    #[must_use]
    pub fn icache_acs_cnt_clr(&mut self) -> ICACHE_ACS_CNT_CLR_W<1> {
        ICACHE_ACS_CNT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_acs_cnt_clr](index.html) module"]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cache_acs_cnt_clr::W](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
