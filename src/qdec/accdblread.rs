#[doc = "Reader of register ACCDBLREAD"]
pub type R = crate::R<u32, super::ACCDBLREAD>;
#[doc = "Reader of field `ACCDBLREAD`"]
pub type ACCDBLREAD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline(always)]
    pub fn accdblread(&self) -> ACCDBLREAD_R {
        ACCDBLREAD_R::new((self.bits & 0x0f) as u8)
    }
}
