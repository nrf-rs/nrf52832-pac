#[allow(unused_imports)]
use vcell::VolatileCell;
#[doc = "Pin number configuration for PDM CLK signal"]
pub struct CLK {
    register: VolatileCell<u32>,
}
#[doc = "Pin number configuration for PDM CLK signal"]
pub mod clk;
#[doc = "Pin number configuration for PDM DIN signal"]
pub struct DIN {
    register: VolatileCell<u32>,
}
#[doc = "Pin number configuration for PDM DIN signal"]
pub mod din;
