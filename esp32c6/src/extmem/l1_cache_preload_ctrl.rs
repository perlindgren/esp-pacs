#[doc = "Register `L1_CACHE_PRELOAD_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_PRELOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_PRELOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_PRELOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_PRELOAD_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_PRELOAD_CTRL_SPEC>;
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
impl From<crate::W<L1_CACHE_PRELOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_PRELOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_PRELOAD_ENA` reader - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done."]
pub type L1_CACHE_PRELOAD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_PRELOAD_ENA` writer - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done."]
pub type L1_CACHE_PRELOAD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_PRELOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_CACHE_PRELOAD_DONE` reader - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
pub type L1_CACHE_PRELOAD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_PRELOAD_ORDER` reader - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L1_CACHE_PRELOAD_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `L1_CACHE_PRELOAD_ORDER` writer - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
pub type L1_CACHE_PRELOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L1_CACHE_PRELOAD_CTRL_SPEC, bool, O>;
#[doc = "Field `L1_CACHE_PRELOAD_RGID` reader - The bit is used to set the gid of l1 cache preload."]
pub type L1_CACHE_PRELOAD_RGID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    pub fn l1_cache_preload_ena(&self) -> L1_CACHE_PRELOAD_ENA_R {
        L1_CACHE_PRELOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether preload operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_cache_preload_done(&self) -> L1_CACHE_PRELOAD_DONE_R {
        L1_CACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    pub fn l1_cache_preload_order(&self) -> L1_CACHE_PRELOAD_ORDER_R {
        L1_CACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of l1 cache preload."]
    #[inline(always)]
    pub fn l1_cache_preload_rgid(&self) -> L1_CACHE_PRELOAD_RGID_R {
        L1_CACHE_PRELOAD_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable preload operation on L1-Cache. It will be cleared by hardware automatically after preload operation is done."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_preload_ena(&mut self) -> L1_CACHE_PRELOAD_ENA_W<0> {
        L1_CACHE_PRELOAD_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of preload operation. 0: ascending, 1: descending."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_preload_order(&mut self) -> L1_CACHE_PRELOAD_ORDER_W<2> {
        L1_CACHE_PRELOAD_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 Cache preload-operation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_preload_ctrl](index.html) module"]
pub struct L1_CACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_preload_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_PRELOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_preload_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_PRELOAD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_PRELOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_CACHE_PRELOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
