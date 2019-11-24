#[doc = "Reader of register ACC"]
pub type R = crate::R<u32, super::ACC>;
#[doc = "Reader of field `ACC`"]
pub type ACC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register accumulating all valid samples (not double transition) read from the SAMPLE register"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
