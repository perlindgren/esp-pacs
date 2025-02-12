#[doc = "Register `BLK0_BACKUP2_W3` reader"]
pub struct R(crate::R<BLK0_BACKUP2_W3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_BACKUP2_W3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_BACKUP2_W3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_BACKUP2_W3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OTP_BEBUG_BLOCK0_BACKUP2_W3` reader - Otp block0 backup2 word3 data."]
pub type OTP_BEBUG_BLOCK0_BACKUP2_W3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup2 word3 data."]
    #[inline(always)]
    pub fn otp_bebug_block0_backup2_w3(&self) -> OTP_BEBUG_BLOCK0_BACKUP2_W3_R {
        OTP_BEBUG_BLOCK0_BACKUP2_W3_R::new(self.bits)
    }
}
#[doc = "Otp debuger block0 data register9.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_backup2_w3](index.html) module"]
pub struct BLK0_BACKUP2_W3_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP2_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_backup2_w3::R](R) reader structure"]
impl crate::Readable for BLK0_BACKUP2_W3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_BACKUP2_W3 to value 0"]
impl crate::Resettable for BLK0_BACKUP2_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
