#[doc = "Register `Core_0_STATUSTABLE4` reader"]
pub struct R(crate::R<CORE_0_STATUSTABLE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_STATUSTABLE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_STATUSTABLE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_STATUSTABLE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_STATUSTABLE4` writer"]
pub struct W(crate::W<CORE_0_STATUSTABLE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_STATUSTABLE4_SPEC>;
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
impl From<crate::W<CORE_0_STATUSTABLE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_STATUSTABLE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_FROM_WORLD_4` reader - This bit is used to confirm world before enter entry 4"]
pub type CORE_0_FROM_WORLD_4_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_FROM_WORLD_4` writer - This bit is used to confirm world before enter entry 4"]
pub type CORE_0_FROM_WORLD_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_STATUSTABLE4_SPEC, bool, O>;
#[doc = "Field `CORE_0_FROM_ENTRY_4` reader - This filed is used to confirm in which entry before enter entry 4"]
pub type CORE_0_FROM_ENTRY_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_FROM_ENTRY_4` writer - This filed is used to confirm in which entry before enter entry 4"]
pub type CORE_0_FROM_ENTRY_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_STATUSTABLE4_SPEC, u8, u8, 4, O>;
#[doc = "Field `CORE_0_CURRENT_4` reader - This bit is used to confirm whether the current state is in entry 4"]
pub type CORE_0_CURRENT_4_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_CURRENT_4` writer - This bit is used to confirm whether the current state is in entry 4"]
pub type CORE_0_CURRENT_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CORE_0_STATUSTABLE4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 4"]
    #[inline(always)]
    pub fn core_0_from_world_4(&self) -> CORE_0_FROM_WORLD_4_R {
        CORE_0_FROM_WORLD_4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 4"]
    #[inline(always)]
    pub fn core_0_from_entry_4(&self) -> CORE_0_FROM_ENTRY_4_R {
        CORE_0_FROM_ENTRY_4_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 4"]
    #[inline(always)]
    pub fn core_0_current_4(&self) -> CORE_0_CURRENT_4_R {
        CORE_0_CURRENT_4_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 4"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_world_4(&mut self) -> CORE_0_FROM_WORLD_4_W<0> {
        CORE_0_FROM_WORLD_4_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 4"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_entry_4(&mut self) -> CORE_0_FROM_ENTRY_4_W<1> {
        CORE_0_FROM_ENTRY_4_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 4"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_current_4(&mut self) -> CORE_0_CURRENT_4_W<5> {
        CORE_0_CURRENT_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of world switch of entry 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_statustable4](index.html) module"]
pub struct CORE_0_STATUSTABLE4_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_statustable4::R](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_statustable4::W](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE4 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
