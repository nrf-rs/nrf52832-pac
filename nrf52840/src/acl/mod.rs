#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - Unspecified"]
    pub acl: [ACL; 8],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct ACL {
    #[doc = "0x00 - Description cluster[n]: Configure the word-aligned start address of region n to protect"]
    pub addr: self::acl::ADDR,
    #[doc = "0x04 - Description cluster[n]: Size of region to protect counting from address ACL[n].ADDR. Write '0' as no effect."]
    pub size: self::acl::SIZE,
    #[doc = "0x08 - Description cluster[n]: Access permissions for region n as defined by start address ACL[n].ADDR and size ACL[n].SIZE"]
    pub perm: self::acl::PERM,
    #[doc = "0x0c - Unspecified"]
    pub unused0: self::acl::UNUSED0,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod acl;
