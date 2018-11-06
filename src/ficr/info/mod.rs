#[doc = "Part code"]
pub struct PART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part code"]
pub mod part;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub struct VARIANT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "Package option"]
pub struct PACKAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Package option"]
pub mod package;
#[doc = "RAM variant"]
pub struct RAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RAM variant"]
pub mod ram;
#[doc = "Flash variant"]
pub struct FLASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash variant"]
pub mod flash;
#[doc = "Description collection[0]: Unspecified"]
pub struct UNUSED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Unspecified"]
pub mod unused0;
