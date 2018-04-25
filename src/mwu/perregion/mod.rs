#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Description cluster[0]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
pub struct SUBSTATWA {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Source of event/interrupt in region 0, write access detected while corresponding subregion was enabled for watching"]
pub mod substatwa;
#[doc = "Description cluster[0]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
pub struct SUBSTATRA {
    register: VolatileCell<u32>,
}
#[doc = "Description cluster[0]: Source of event/interrupt in region 0, read access detected while corresponding subregion was enabled for watching"]
pub mod substatra;
