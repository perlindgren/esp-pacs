#[doc = "Register `INT_RAW_CH0` reader"]
pub struct R(crate::R<INT_RAW_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW_CH0` writer"]
pub struct W(crate::W<INT_RAW_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_CH0_SPEC>;
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
impl From<crate::W<INT_RAW_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type IN_SUC_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
pub type IN_ERR_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF` writer - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
pub type IN_ERR_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
pub type IN_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY` writer - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
pub type IN_DSCR_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `INFIFO_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type INFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `INFIFO_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type INFIFO_UDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_CH0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_ovf(&self) -> INFIFO_OVF_R {
        INFIFO_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_udf(&self) -> INFIFO_UDF_R {
        INFIFO_UDF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf(&self) -> OUTFIFO_OVF_R {
        OUTFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf(&self) -> OUTFIFO_UDF_R {
        OUTFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<0> {
        IN_DONE_W::new(self)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0, the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<1> {
        IN_SUC_EOF_W::new(self)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals, this raw interrupt is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<2> {
        IN_ERR_EOF_W::new(self)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<3> {
        OUT_DONE_W::new(self)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<4> {
        OUT_EOF_W::new(self)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when detecting inlink descriptor error, including owner error, the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<5> {
        IN_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<6> {
        OUT_DSCR_ERR_W::new(self)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed, but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<7> {
        IN_DSCR_EMPTY_W::new(self)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<8> {
        OUT_TOTAL_EOF_W::new(self)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf(&mut self) -> INFIFO_OVF_W<9> {
        INFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 10 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf(&mut self) -> INFIFO_UDF_W<10> {
        INFIFO_UDF_W::new(self)
    }
    #[doc = "Bit 11 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<11> {
        OUTFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 12 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<12> {
        OUTFIFO_UDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INT_RAW_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw_ch0](index.html) module"]
pub struct INT_RAW_CH0_SPEC;
impl crate::RegisterSpec for INT_RAW_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw_ch0::R](R) reader structure"]
impl crate::Readable for INT_RAW_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw_ch0::W](W) writer structure"]
impl crate::Writable for INT_RAW_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW_CH0 to value 0"]
impl crate::Resettable for INT_RAW_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
