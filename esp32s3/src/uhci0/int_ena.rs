#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TX_START_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_START_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_HUNG_INT_ENA` writer - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub type SEND_S_REG_Q_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SEND_S_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub type SEND_S_REG_Q_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` reader - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub type SEND_A_REG_Q_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SEND_A_REG_Q_INT_ENA` writer - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub type SEND_A_REG_Q_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `APP_CTRL0_INT_ENA` reader - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub type APP_CTRL0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL0_INT_ENA` writer - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub type APP_CTRL0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `APP_CTRL1_INT_ENA` reader - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub type APP_CTRL1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL1_INT_ENA` writer - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub type APP_CTRL1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q_int_ena(&self) -> SEND_S_REG_Q_INT_ENA_R {
        SEND_S_REG_Q_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q_int_ena(&self) -> SEND_A_REG_Q_INT_ENA_R {
        SEND_A_REG_Q_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> APP_CTRL0_INT_ENA_R {
        APP_CTRL0_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&self) -> APP_CTRL1_INT_ENA_R {
        APP_CTRL1_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W<0> {
        RX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W<1> {
        TX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<2> {
        RX_HUNG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W<3> {
        TX_HUNG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q_int_ena(&mut self) -> SEND_S_REG_Q_INT_ENA_W<4> {
        SEND_S_REG_Q_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q_int_ena(&mut self) -> SEND_A_REG_Q_INT_ENA_W<5> {
        SEND_A_REG_Q_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W<6> {
        OUTLINK_EOF_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0_int_ena(&mut self) -> APP_CTRL0_INT_ENA_W<7> {
        APP_CTRL0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1_int_ena(&mut self) -> APP_CTRL1_INT_ENA_W<8> {
        APP_CTRL1_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
