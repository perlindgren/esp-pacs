#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_HOLD` reader - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub type CS_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `CS_HOLD` writer - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
pub type CS_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `CS_SETUP` reader - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub type CS_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `CS_SETUP` writer - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
pub type CS_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `CK_OUT_EDGE` reader - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
pub type CK_OUT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CK_OUT_EDGE` writer - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
pub type CK_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DUMMY_IDLE` reader - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
pub type USR_DUMMY_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY_IDLE` writer - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
pub type USR_DUMMY_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the DUMMY phase of an SPI transfer."]
pub type USR_DUMMY_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY` writer - This bit enable the DUMMY phase of an SPI transfer."]
pub type USR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26 - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the DUMMY phase of an SPI transfer."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Set this bit to keep SPI_CS low when MSPI is in DONE state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<6> {
        CS_HOLD_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to keep SPI_CS low when MSPI is in PREP state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<7> {
        CS_SETUP_W::new(self)
    }
    #[doc = "Bit 9 - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<9> {
        CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 26 - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<26> {
        USR_DUMMY_IDLE_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the DUMMY phase of an SPI transfer."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<29> {
        USR_DUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 user register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
