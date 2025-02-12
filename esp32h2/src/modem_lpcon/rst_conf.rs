#[doc = "Register `RST_CONF` writer"]
pub struct W(crate::W<RST_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_CONF_SPEC>;
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
impl From<crate::W<RST_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST_COEX` writer - "]
pub type RST_COEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_CONF_SPEC, bool, O>;
#[doc = "Field `RST_I2C_MST` writer - "]
pub type RST_I2C_MST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_CONF_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rst_coex(&mut self) -> RST_COEX_W<1> {
        RST_COEX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rst_i2c_mst(&mut self) -> RST_I2C_MST_W<2> {
        RST_I2C_MST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_conf](index.html) module"]
pub struct RST_CONF_SPEC;
impl crate::RegisterSpec for RST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rst_conf::W](W) writer structure"]
impl crate::Writable for RST_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST_CONF to value 0"]
impl crate::Resettable for RST_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
