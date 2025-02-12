#[doc = "Register `CORE_RST_EN` reader"]
pub struct R(crate::R<CORE_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_RST_EN` writer"]
pub struct W(crate::W<CORE_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_RST_EN_SPEC>;
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
impl From<crate::W<CORE_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_RST` reader - "]
pub type CORE_RST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_RST` writer - "]
pub type CORE_RST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORE_RST_EN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&self) -> CORE_RST_R {
        CORE_RST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn core_rst(&mut self) -> CORE_RST_W<0> {
        CORE_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_rst_en](index.html) module"]
pub struct CORE_RST_EN_SPEC;
impl crate::RegisterSpec for CORE_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_rst_en::R](R) reader structure"]
impl crate::Readable for CORE_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_rst_en::W](W) writer structure"]
impl crate::Writable for CORE_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_RST_EN to value 0"]
impl crate::Resettable for CORE_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
