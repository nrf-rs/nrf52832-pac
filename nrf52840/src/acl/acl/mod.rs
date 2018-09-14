#[doc = "Description cluster[n]: Configure the word-aligned start address of region n to protect"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Configure the word-aligned start address of region n to protect"]
pub mod addr;
#[doc = "Description cluster[n]: Size of region to protect counting from address ACL[n].ADDR. Write '0' as no effect."]
pub struct SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Size of region to protect counting from address ACL[n].ADDR. Write '0' as no effect."]
pub mod size;
#[doc = "Description cluster[n]: Access permissions for region n as defined by start address ACL[n].ADDR and size ACL[n].SIZE"]
pub struct PERM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster[n]: Access permissions for region n as defined by start address ACL[n].ADDR and size ACL[n].SIZE"]
pub mod perm;
#[doc = "Unspecified"]
pub struct UNUSED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused0;
