#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_START_INT_ST` reader - Indicates the interrupt status of UHCI_RX_START_INT."]
pub type RX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TX_START_INT_ST` reader - Indicates the interrupt status of UHCI_TX_START_INT."]
pub type TX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `RX_HUNG_INT_ST` reader - Indicates the interrupt status of UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `TX_HUNG_INT_ST` reader - Indicates the interrupt status of UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SEND_S_REG_Q_INT_ST` reader - Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `SEND_A_REG_Q_INT_ST` reader - Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ST` reader - Indicates the interrupt status of UHCI_OUT_EOF_INT."]
pub type OUTLINK_EOF_ERR_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL0_INT_ST` reader - Indicates the interrupt status of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `APP_CTRL1_INT_ST` reader - Indicates the interrupt status of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the interrupt status of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_st(&self) -> RX_START_INT_ST_R {
        RX_START_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the interrupt status of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_st(&self) -> TX_START_INT_ST_R {
        TX_START_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the interrupt status of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the interrupt status of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates the interrupt status of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_st(&self) -> SEND_S_REG_Q_INT_ST_R {
        SEND_S_REG_Q_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates the interrupt status of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_st(&self) -> SEND_A_REG_Q_INT_ST_R {
        SEND_A_REG_Q_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates the interrupt status of UHCI_OUT_EOF_INT."]
    #[inline(always)]
    pub fn outlink_eof_err_int_st(&self) -> OUTLINK_EOF_ERR_INT_ST_R {
        OUTLINK_EOF_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the interrupt status of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_st(&self) -> APP_CTRL0_INT_ST_R {
        APP_CTRL0_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the interrupt status of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_st(&self) -> APP_CTRL1_INT_ST_R {
        APP_CTRL1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "UHCI Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
