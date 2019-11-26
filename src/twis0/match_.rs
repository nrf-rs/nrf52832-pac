#[doc = "Reader of register MATCH"]
pub type R = crate::R<u32, super::MATCH>;
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Which of the addresses in {ADDRESS} matched the incoming address"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0x01) != 0)
    }
}
