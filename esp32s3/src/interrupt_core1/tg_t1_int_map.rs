#[doc = "Register `TG_T1_INT_MAP` reader"]
pub struct R(crate::R<TG_T1_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TG_T1_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TG_T1_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TG_T1_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TG_T1_INT_MAP` writer"]
pub struct W(crate::W<TG_T1_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TG_T1_INT_MAP_SPEC>;
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
impl From<crate::W<TG_T1_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TG_T1_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TG_T1_INT_MAP` reader - this register used to map tg_t1 interrupt to one of core1's external interrupt"]
pub type TG_T1_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TG_T1_INT_MAP` writer - this register used to map tg_t1 interrupt to one of core1's external interrupt"]
pub type TG_T1_INT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TG_T1_INT_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map tg_t1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn tg_t1_int_map(&self) -> TG_T1_INT_MAP_R {
        TG_T1_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map tg_t1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tg_t1_int_map(&mut self) -> TG_T1_INT_MAP_W<0> {
        TG_T1_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tg_t1 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tg_t1_int_map](index.html) module"]
pub struct TG_T1_INT_MAP_SPEC;
impl crate::RegisterSpec for TG_T1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tg_t1_int_map::R](R) reader structure"]
impl crate::Readable for TG_T1_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tg_t1_int_map::W](W) writer structure"]
impl crate::Writable for TG_T1_INT_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TG_T1_INT_MAP to value 0x10"]
impl crate::Resettable for TG_T1_INT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
