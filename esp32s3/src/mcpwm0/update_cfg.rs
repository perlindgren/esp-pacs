#[doc = "Register `UPDATE_CFG` reader"]
pub struct R(crate::R<UPDATE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPDATE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPDATE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPDATE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPDATE_CFG` writer"]
pub struct W(crate::W<UPDATE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPDATE_CFG_SPEC>;
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
impl From<crate::W<UPDATE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPDATE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLOBAL_UP_EN` reader - The global enable of update of all active registers in MCPWM module"]
pub type GLOBAL_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `GLOBAL_UP_EN` writer - The global enable of update of all active registers in MCPWM module"]
pub type GLOBAL_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `GLOBAL_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub type GLOBAL_FORCE_UP_R = crate::BitReader<bool>;
#[doc = "Field `GLOBAL_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub type GLOBAL_FORCE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP0_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
pub type OP0_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OP0_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
pub type OP0_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP0_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
pub type OP0_FORCE_UP_R = crate::BitReader<bool>;
#[doc = "Field `OP0_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
pub type OP0_FORCE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP1_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
pub type OP1_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OP1_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
pub type OP1_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP1_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
pub type OP1_FORCE_UP_R = crate::BitReader<bool>;
#[doc = "Field `OP1_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
pub type OP1_FORCE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP2_UP_EN` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
pub type OP2_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OP2_UP_EN` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
pub type OP2_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
#[doc = "Field `OP2_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
pub type OP2_FORCE_UP_R = crate::BitReader<bool>;
#[doc = "Field `OP2_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
pub type OP2_FORCE_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPDATE_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GLOBAL_UP_EN_R {
        GLOBAL_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&self) -> GLOBAL_FORCE_UP_R {
        GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> OP0_UP_EN_R {
        OP0_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&self) -> OP0_FORCE_UP_R {
        OP0_FORCE_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> OP1_UP_EN_R {
        OP1_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&self) -> OP1_FORCE_UP_R {
        OP1_FORCE_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> OP2_UP_EN_R {
        OP2_UP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&self) -> OP2_FORCE_UP_R {
        OP2_FORCE_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    #[must_use]
    pub fn global_up_en(&mut self) -> GLOBAL_UP_EN_W<0> {
        GLOBAL_UP_EN_W::new(self)
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    #[must_use]
    pub fn global_force_up(&mut self) -> GLOBAL_FORCE_UP_W<1> {
        GLOBAL_FORCE_UP_W::new(self)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn op0_up_en(&mut self) -> OP0_UP_EN_W<2> {
        OP0_UP_EN_W::new(self)
    }
    #[doc = "Bit 3 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    #[must_use]
    pub fn op0_force_up(&mut self) -> OP0_FORCE_UP_W<3> {
        OP0_FORCE_UP_W::new(self)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn op1_up_en(&mut self) -> OP1_UP_EN_W<4> {
        OP1_UP_EN_W::new(self)
    }
    #[doc = "Bit 5 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    #[must_use]
    pub fn op1_force_up(&mut self) -> OP1_FORCE_UP_W<5> {
        OP1_FORCE_UP_W::new(self)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    #[must_use]
    pub fn op2_up_en(&mut self) -> OP2_UP_EN_W<6> {
        OP2_UP_EN_W::new(self)
    }
    #[doc = "Bit 7 - a toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    #[must_use]
    pub fn op2_force_up(&mut self) -> OP2_FORCE_UP_W<7> {
        OP2_FORCE_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable update.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_cfg](index.html) module"]
pub struct UPDATE_CFG_SPEC;
impl crate::RegisterSpec for UPDATE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [update_cfg::R](R) reader structure"]
impl crate::Readable for UPDATE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [update_cfg::W](W) writer structure"]
impl crate::Writable for UPDATE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPDATE_CFG to value 0x55"]
impl crate::Resettable for UPDATE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}
