#[doc = "Description collection[n]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub struct EPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epin;
#[doc = "Description collection[n]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub struct EPOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epout;
