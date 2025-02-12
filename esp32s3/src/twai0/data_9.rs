#[doc = "Register `DATA_9` reader"]
pub struct R(crate::R<DATA_9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_9` writer"]
pub struct W(crate::W<DATA_9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_9_SPEC>;
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
impl From<crate::W<DATA_9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_9` reader - Stored the 9th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_BYTE_9` writer - Stored the 9th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_9_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Stored the 9th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_9(&self) -> TX_BYTE_9_R {
        TX_BYTE_9_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Stored the 9th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_9(&mut self) -> TX_BYTE_9_W<0> {
        TX_BYTE_9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_9](index.html) module"]
pub struct DATA_9_SPEC;
impl crate::RegisterSpec for DATA_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_9::R](R) reader structure"]
impl crate::Readable for DATA_9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_9::W](W) writer structure"]
impl crate::Writable for DATA_9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_9 to value 0"]
impl crate::Resettable for DATA_9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
