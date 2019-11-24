#[doc = "Reader of register IR[%s]"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Reader of field `IR`"]
pub type IR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Identity Root, word n"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
