#[doc = "Register `FH2_CFG0` reader"]
pub struct R(crate::R<FH2_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH2_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH2_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH2_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH2_CFG0` writer"]
pub struct W(crate::W<FH2_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH2_CFG0_SPEC>;
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
impl From<crate::W<FH2_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH2_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ2_SW_CBC` reader - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_SW_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_SW_CBC` writer - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_SW_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F2_CBC` reader - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F2_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F2_CBC` writer - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F2_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F1_CBC` reader - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F1_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F1_CBC` writer - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F1_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F0_CBC` reader - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F0_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F0_CBC` writer - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
pub type TZ2_F0_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_SW_OST` reader - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_SW_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_SW_OST` writer - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_SW_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F2_OST` reader - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F2_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F2_OST` writer - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F2_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F1_OST` reader - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F1_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F1_OST` writer - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F1_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_F0_OST` reader - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F0_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ2_F0_OST` writer - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
pub type TZ2_F0_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG0_SPEC, bool, O>;
#[doc = "Field `TZ2_A_CBC_D` reader - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_A_CBC_D` writer - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_A_CBC_U` reader - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_A_CBC_U` writer - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_A_OST_D` reader - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_A_OST_D` writer - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_A_OST_U` reader - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_A_OST_U` writer - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_A_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_B_CBC_D` reader - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_CBC_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_B_CBC_D` writer - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_CBC_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_B_CBC_U` reader - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_CBC_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_B_CBC_U` writer - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_CBC_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_B_OST_D` reader - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_OST_D_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_B_OST_D` writer - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_OST_D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ2_B_OST_U` reader - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_OST_U_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ2_B_OST_U` writer - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
pub type TZ2_B_OST_U_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_cbc(&self) -> TZ2_SW_CBC_R {
        TZ2_SW_CBC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_cbc(&self) -> TZ2_F2_CBC_R {
        TZ2_F2_CBC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_cbc(&self) -> TZ2_F1_CBC_R {
        TZ2_F1_CBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_cbc(&self) -> TZ2_F0_CBC_R {
        TZ2_F0_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_sw_ost(&self) -> TZ2_SW_OST_R {
        TZ2_SW_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f2_ost(&self) -> TZ2_F2_OST_R {
        TZ2_F2_OST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f1_ost(&self) -> TZ2_F1_OST_R {
        TZ2_F1_OST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tz2_f0_ost(&self) -> TZ2_F0_OST_R {
        TZ2_F0_OST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_d(&self) -> TZ2_A_CBC_D_R {
        TZ2_A_CBC_D_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_cbc_u(&self) -> TZ2_A_CBC_U_R {
        TZ2_A_CBC_U_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_d(&self) -> TZ2_A_OST_D_R {
        TZ2_A_OST_D_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_a_ost_u(&self) -> TZ2_A_OST_U_R {
        TZ2_A_OST_U_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_d(&self) -> TZ2_B_CBC_D_R {
        TZ2_B_CBC_D_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_cbc_u(&self) -> TZ2_B_CBC_U_R {
        TZ2_B_CBC_U_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_d(&self) -> TZ2_B_OST_D_R {
        TZ2_B_OST_D_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    pub fn tz2_b_ost_u(&self) -> TZ2_B_OST_U_R {
        TZ2_B_OST_U_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable register for software force cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_sw_cbc(&mut self) -> TZ2_SW_CBC_W<0> {
        TZ2_SW_CBC_W::new(self)
    }
    #[doc = "Bit 1 - event_f2 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f2_cbc(&mut self) -> TZ2_F2_CBC_W<1> {
        TZ2_F2_CBC_W::new(self)
    }
    #[doc = "Bit 2 - event_f1 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f1_cbc(&mut self) -> TZ2_F1_CBC_W<2> {
        TZ2_F1_CBC_W::new(self)
    }
    #[doc = "Bit 3 - event_f0 will trigger cycle-by-cycle mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f0_cbc(&mut self) -> TZ2_F0_CBC_W<3> {
        TZ2_F0_CBC_W::new(self)
    }
    #[doc = "Bit 4 - Enable register for software force one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_sw_ost(&mut self) -> TZ2_SW_OST_W<4> {
        TZ2_SW_OST_W::new(self)
    }
    #[doc = "Bit 5 - event_f2 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f2_ost(&mut self) -> TZ2_F2_OST_W<5> {
        TZ2_F2_OST_W::new(self)
    }
    #[doc = "Bit 6 - event_f1 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f1_ost(&mut self) -> TZ2_F1_OST_W<6> {
        TZ2_F1_OST_W::new(self)
    }
    #[doc = "Bit 7 - event_f0 will trigger one-shot mode action. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_f0_ost(&mut self) -> TZ2_F0_OST_W<7> {
        TZ2_F0_OST_W::new(self)
    }
    #[doc = "Bits 8:9 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_a_cbc_d(&mut self) -> TZ2_A_CBC_D_W<8> {
        TZ2_A_CBC_D_W::new(self)
    }
    #[doc = "Bits 10:11 - Cycle-by-cycle mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_a_cbc_u(&mut self) -> TZ2_A_CBC_U_W<10> {
        TZ2_A_CBC_U_W::new(self)
    }
    #[doc = "Bits 12:13 - One-shot mode action on PWM2A when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_a_ost_d(&mut self) -> TZ2_A_OST_D_W<12> {
        TZ2_A_OST_D_W::new(self)
    }
    #[doc = "Bits 14:15 - One-shot mode action on PWM2A when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_a_ost_u(&mut self) -> TZ2_A_OST_U_W<14> {
        TZ2_A_OST_U_W::new(self)
    }
    #[doc = "Bits 16:17 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_b_cbc_d(&mut self) -> TZ2_B_CBC_D_W<16> {
        TZ2_B_CBC_D_W::new(self)
    }
    #[doc = "Bits 18:19 - Cycle-by-cycle mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_b_cbc_u(&mut self) -> TZ2_B_CBC_U_W<18> {
        TZ2_B_CBC_U_W::new(self)
    }
    #[doc = "Bits 20:21 - One-shot mode action on PWM2B when fault event occurs and timer is decreasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_b_ost_d(&mut self) -> TZ2_B_OST_D_W<20> {
        TZ2_B_OST_D_W::new(self)
    }
    #[doc = "Bits 22:23 - One-shot mode action on PWM2B when fault event occurs and timer is increasing. 0: do nothing, 1: force low, 2: force high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn tz2_b_ost_u(&mut self) -> TZ2_B_OST_U_W<22> {
        TZ2_B_OST_U_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions on PWM2A and PWM2B trip events\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh2_cfg0](index.html) module"]
pub struct FH2_CFG0_SPEC;
impl crate::RegisterSpec for FH2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh2_cfg0::R](R) reader structure"]
impl crate::Readable for FH2_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh2_cfg0::W](W) writer structure"]
impl crate::Writable for FH2_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FH2_CFG0 to value 0"]
impl crate::Resettable for FH2_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
