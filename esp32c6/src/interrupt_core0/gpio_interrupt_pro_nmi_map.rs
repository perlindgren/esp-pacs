#[doc = "Register `GPIO_INTERRUPT_PRO_NMI_MAP` reader"]
pub struct R(crate::R<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTERRUPT_PRO_NMI_MAP` writer"]
pub struct W(crate::W<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
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
impl From<crate::W<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_INTERRUPT_PRO_NMI_MAP` reader - Need add description"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_INTERRUPT_PRO_NMI_MAP` writer - Need add description"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_INTERRUPT_PRO_NMI_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn gpio_interrupt_pro_nmi_map(&self) -> GPIO_INTERRUPT_PRO_NMI_MAP_R {
        GPIO_INTERRUPT_PRO_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_interrupt_pro_nmi_map(&mut self) -> GPIO_INTERRUPT_PRO_NMI_MAP_W<0> {
        GPIO_INTERRUPT_PRO_NMI_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_interrupt_pro_nmi_map](index.html) module"]
pub struct GPIO_INTERRUPT_PRO_NMI_MAP_SPEC;
impl crate::RegisterSpec for GPIO_INTERRUPT_PRO_NMI_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_interrupt_pro_nmi_map::R](R) reader structure"]
impl crate::Readable for GPIO_INTERRUPT_PRO_NMI_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_interrupt_pro_nmi_map::W](W) writer structure"]
impl crate::Writable for GPIO_INTERRUPT_PRO_NMI_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INTERRUPT_PRO_NMI_MAP to value 0"]
impl crate::Resettable for GPIO_INTERRUPT_PRO_NMI_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
