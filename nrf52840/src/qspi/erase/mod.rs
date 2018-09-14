#[doc = "Start address of flash block to be erased"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start address of flash block to be erased"]
pub mod ptr;
#[doc = "Size of block to be erased."]
pub struct LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Size of block to be erased."]
pub mod len;
