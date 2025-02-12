#[doc = "Register `L1_ICACHE1_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<L1_ICACHE1_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE1_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE1_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE1_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE1_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-ICache1. 1: enable, 0: disable."]
pub type L1_ICACHE1_AUTOLOAD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-ICache1 is finished or not. 0: not finished. 1: finished."]
pub type L1_ICACHE1_AUTOLOAD_DONE_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-ICache1. 0: ascending. 1: descending."]
pub type L1_ICACHE1_AUTOLOAD_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-ICache1. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_ICACHE1_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-ICache1."]
pub type L1_ICACHE1_AUTOLOAD_SCT0_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-ICache1."]
pub type L1_ICACHE1_AUTOLOAD_SCT1_ENA_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 icache1 autoload."]
pub type L1_ICACHE1_AUTOLOAD_RGID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-ICache1. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_icache1_autoload_ena(&self) -> L1_ICACHE1_AUTOLOAD_ENA_R {
        L1_ICACHE1_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-ICache1 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_icache1_autoload_done(&self) -> L1_ICACHE1_AUTOLOAD_DONE_R {
        L1_ICACHE1_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-ICache1. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_icache1_autoload_order(&self) -> L1_ICACHE1_AUTOLOAD_ORDER_R {
        L1_ICACHE1_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-ICache1. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_icache1_autoload_trigger_mode(&self) -> L1_ICACHE1_AUTOLOAD_TRIGGER_MODE_R {
        L1_ICACHE1_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_autoload_sct0_ena(&self) -> L1_ICACHE1_AUTOLOAD_SCT0_ENA_R {
        L1_ICACHE1_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_autoload_sct1_ena(&self) -> L1_ICACHE1_AUTOLOAD_SCT1_ENA_R {
        L1_ICACHE1_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - The bit is used to set the gid of l1 icache1 autoload."]
    #[inline(always)]
    pub fn l1_icache1_autoload_rgid(&self) -> L1_ICACHE1_AUTOLOAD_RGID_R {
        L1_ICACHE1_AUTOLOAD_RGID_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
#[doc = "L1 instruction Cache 1 autoload-operation control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache1_autoload_ctrl](index.html) module"]
pub struct L1_ICACHE1_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE1_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache1_autoload_ctrl::R](R) reader structure"]
impl crate::Readable for L1_ICACHE1_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE1_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_ICACHE1_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
