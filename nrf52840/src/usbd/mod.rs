#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Description collection[n]: Captures the EPIN[n].PTR and EPIN[n].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    pub tasks_startepin: [TASKS_STARTEPIN; 8],
    #[doc = "0x24 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    pub tasks_startisoin: TASKS_STARTISOIN,
    #[doc = "0x28 - Description collection[n]: Captures the EPOUT[n].PTR and EPOUT[n].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    pub tasks_startepout: [TASKS_STARTEPOUT; 8],
    #[doc = "0x48 - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    pub tasks_startisoout: TASKS_STARTISOOUT,
    #[doc = "0x4c - Allows OUT data stage on control endpoint 0"]
    pub tasks_ep0rcvout: TASKS_EP0RCVOUT,
    #[doc = "0x50 - Allows status stage on control endpoint 0"]
    pub tasks_ep0status: TASKS_EP0STATUS,
    #[doc = "0x54 - Stalls data and status stage on control endpoint 0"]
    pub tasks_ep0stall: TASKS_EP0STALL,
    #[doc = "0x58 - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    pub tasks_dpdmdrive: TASKS_DPDMDRIVE,
    #[doc = "0x5c - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    pub tasks_dpdmnodrive: TASKS_DPDMNODRIVE,
    _reserved9: [u8; 160usize],
    #[doc = "0x100 - Signals that a USB reset condition has been detected on USB lines"]
    pub events_usbreset: EVENTS_USBRESET,
    #[doc = "0x104 - Confirms that the EPIN[n].PTR and EPIN[n].MAXCNT, or EPOUT[n].PTR and EPOUT[n].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x108 - Description collection[n]: The whole EPIN[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepin: [EVENTS_ENDEPIN; 8],
    #[doc = "0x128 - An acknowledged data transfer has taken place on the control endpoint"]
    pub events_ep0datadone: EVENTS_EP0DATADONE,
    #[doc = "0x12c - The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoin: EVENTS_ENDISOIN,
    #[doc = "0x130 - Description collection[n]: The whole EPOUT[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepout: [EVENTS_ENDEPOUT; 8],
    #[doc = "0x150 - The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoout: EVENTS_ENDISOOUT,
    #[doc = "0x154 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    pub events_sof: EVENTS_SOF,
    #[doc = "0x158 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    pub events_usbevent: EVENTS_USBEVENT,
    #[doc = "0x15c - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    pub events_ep0setup: EVENTS_EP0SETUP,
    #[doc = "0x160 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    pub events_epdata: EVENTS_EPDATA,
    _reserved20: [u8; 156usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved21: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved24: [u8; 244usize],
    #[doc = "0x400 - Details on what caused the USBEVENT event"]
    pub eventcause: EVENTCAUSE,
    _reserved25: [u8; 28usize],
    #[doc = "0x420 - Unspecified"]
    pub halted: HALTED,
    _reserved26: [u8; 4usize],
    #[doc = "0x468 - Provides information on which endpoint's EasyDMA registers have been captured"]
    pub epstatus: EPSTATUS,
    #[doc = "0x46c - Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    pub epdatastatus: EPDATASTATUS,
    #[doc = "0x470 - Device USB address"]
    pub usbaddr: USBADDR,
    _reserved29: [u8; 12usize],
    #[doc = "0x480 - SETUP data, byte 0, bmRequestType"]
    pub bmrequesttype: BMREQUESTTYPE,
    #[doc = "0x484 - SETUP data, byte 1, bRequest"]
    pub brequest: BREQUEST,
    #[doc = "0x488 - SETUP data, byte 2, LSB of wValue"]
    pub wvaluel: WVALUEL,
    #[doc = "0x48c - SETUP data, byte 3, MSB of wValue"]
    pub wvalueh: WVALUEH,
    #[doc = "0x490 - SETUP data, byte 4, LSB of wIndex"]
    pub windexl: WINDEXL,
    #[doc = "0x494 - SETUP data, byte 5, MSB of wIndex"]
    pub windexh: WINDEXH,
    #[doc = "0x498 - SETUP data, byte 6, LSB of wLength"]
    pub wlengthl: WLENGTHL,
    #[doc = "0x49c - SETUP data, byte 7, MSB of wLength"]
    pub wlengthh: WLENGTHH,
    #[doc = "0x4a0 - Unspecified"]
    pub size: SIZE,
    _reserved38: [u8; 60usize],
    #[doc = "0x500 - Enable USB"]
    pub enable: ENABLE,
    #[doc = "0x504 - Control of the USB pull-up"]
    pub usbpullup: USBPULLUP,
    #[doc = "0x508 - State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    pub dpdmvalue: DPDMVALUE,
    #[doc = "0x50c - Data toggle control and status"]
    pub dtoggle: DTOGGLE,
    #[doc = "0x510 - Endpoint IN enable"]
    pub epinen: EPINEN,
    #[doc = "0x514 - Endpoint OUT enable"]
    pub epouten: EPOUTEN,
    #[doc = "0x518 - STALL endpoints"]
    pub epstall: EPSTALL,
    #[doc = "0x51c - Controls the split of ISO buffers"]
    pub isosplit: ISOSPLIT,
    #[doc = "0x520 - Returns the current value of the start of frame counter"]
    pub framecntr: FRAMECNTR,
    _reserved47: [u8; 8usize],
    #[doc = "0x52c - Controls USBD peripheral low power mode during USB suspend"]
    pub lowpower: LOWPOWER,
    #[doc = "0x530 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    pub isoinconfig: ISOINCONFIG,
    _reserved49: [u8; 204usize],
    #[doc = "0x600 - Unspecified"]
    pub epin0: EPIN,
    _reserved50: [u8; 8usize],
    #[doc = "0x614 - Unspecified"]
    pub epin1: EPIN,
    _reserved51: [u8; 8usize],
    #[doc = "0x628 - Unspecified"]
    pub epin2: EPIN,
    _reserved52: [u8; 8usize],
    #[doc = "0x63c - Unspecified"]
    pub epin3: EPIN,
    _reserved53: [u8; 8usize],
    #[doc = "0x650 - Unspecified"]
    pub epin4: EPIN,
    _reserved54: [u8; 8usize],
    #[doc = "0x664 - Unspecified"]
    pub epin5: EPIN,
    _reserved55: [u8; 8usize],
    #[doc = "0x678 - Unspecified"]
    pub epin6: EPIN,
    _reserved56: [u8; 8usize],
    #[doc = "0x68c - Unspecified"]
    pub epin7: EPIN,
    _reserved57: [u8; 8usize],
    #[doc = "0x6a0 - Unspecified"]
    pub isoin: ISOIN,
    _reserved58: [u8; 84usize],
    #[doc = "0x700 - Unspecified"]
    pub epout0: EPOUT,
    _reserved59: [u8; 8usize],
    #[doc = "0x714 - Unspecified"]
    pub epout1: EPOUT,
    _reserved60: [u8; 8usize],
    #[doc = "0x728 - Unspecified"]
    pub epout2: EPOUT,
    _reserved61: [u8; 8usize],
    #[doc = "0x73c - Unspecified"]
    pub epout3: EPOUT,
    _reserved62: [u8; 8usize],
    #[doc = "0x750 - Unspecified"]
    pub epout4: EPOUT,
    _reserved63: [u8; 8usize],
    #[doc = "0x764 - Unspecified"]
    pub epout5: EPOUT,
    _reserved64: [u8; 8usize],
    #[doc = "0x778 - Unspecified"]
    pub epout6: EPOUT,
    _reserved65: [u8; 8usize],
    #[doc = "0x78c - Unspecified"]
    pub epout7: EPOUT,
    _reserved66: [u8; 8usize],
    #[doc = "0x7a0 - Unspecified"]
    pub isoout: ISOOUT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct HALTED {
    #[doc = "0x00 - Description collection[n]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epin: [self::halted::EPIN; 8],
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Description collection[n]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epout: [self::halted::EPOUT; 8],
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod halted;
#[doc = r" Register block"]
#[repr(C)]
pub struct SIZE {
    #[doc = "0x00 - Description collection[n]: Number of bytes received last in the data stage of this OUT endpoint"]
    pub epout: [self::size::EPOUT; 8],
    #[doc = "0x20 - Number of bytes received last on this ISO OUT data endpoint"]
    pub isoout: self::size::ISOOUT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod size;
#[doc = r" Register block"]
#[repr(C)]
pub struct EPIN {
    #[doc = "0x00 - Description cluster[n]: Data pointer"]
    pub ptr: self::epin::PTR,
    #[doc = "0x04 - Description cluster[n]: Maximum number of bytes to transfer"]
    pub maxcnt: self::epin::MAXCNT,
    #[doc = "0x08 - Description cluster[n]: Number of bytes transferred in the last transaction"]
    pub amount: self::epin::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod epin;
#[doc = r" Register block"]
#[repr(C)]
pub struct ISOIN {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::isoin::PTR,
    #[doc = "0x04 - Maximum number of bytes to transfer"]
    pub maxcnt: self::isoin::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::isoin::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod isoin;
#[doc = r" Register block"]
#[repr(C)]
pub struct EPOUT {
    #[doc = "0x00 - Description cluster[n]: Data pointer"]
    pub ptr: self::epout::PTR,
    #[doc = "0x04 - Description cluster[n]: Maximum number of bytes to transfer"]
    pub maxcnt: self::epout::MAXCNT,
    #[doc = "0x08 - Description cluster[n]: Number of bytes transferred in the last transaction"]
    pub amount: self::epout::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod epout;
#[doc = r" Register block"]
#[repr(C)]
pub struct ISOOUT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::isoout::PTR,
    #[doc = "0x04 - Maximum number of bytes to transfer"]
    pub maxcnt: self::isoout::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::isoout::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod isoout;
#[doc = "Description collection[n]: Captures the EPIN[n].PTR and EPIN[n].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub struct TASKS_STARTEPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Captures the EPIN[n].PTR and EPIN[n].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub mod tasks_startepin;
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub struct TASKS_STARTISOIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub mod tasks_startisoin;
#[doc = "Description collection[n]: Captures the EPOUT[n].PTR and EPOUT[n].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub struct TASKS_STARTEPOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Captures the EPOUT[n].PTR and EPOUT[n].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub mod tasks_startepout;
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub struct TASKS_STARTISOOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub mod tasks_startisoout;
#[doc = "Allows OUT data stage on control endpoint 0"]
pub struct TASKS_EP0RCVOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows OUT data stage on control endpoint 0"]
pub mod tasks_ep0rcvout;
#[doc = "Allows status stage on control endpoint 0"]
pub struct TASKS_EP0STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Allows status stage on control endpoint 0"]
pub mod tasks_ep0status;
#[doc = "Stalls data and status stage on control endpoint 0"]
pub struct TASKS_EP0STALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stalls data and status stage on control endpoint 0"]
pub mod tasks_ep0stall;
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub struct TASKS_DPDMDRIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub mod tasks_dpdmdrive;
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub struct TASKS_DPDMNODRIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub mod tasks_dpdmnodrive;
#[doc = "Signals that a USB reset condition has been detected on USB lines"]
pub struct EVENTS_USBRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signals that a USB reset condition has been detected on USB lines"]
pub mod events_usbreset;
#[doc = "Confirms that the EPIN[n].PTR and EPIN[n].MAXCNT, or EPOUT[n].PTR and EPOUT[n].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Confirms that the EPIN[n].PTR and EPIN[n].MAXCNT, or EPOUT[n].PTR and EPOUT[n].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
pub mod events_started;
#[doc = "Description collection[n]: The whole EPIN[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub struct EVENTS_ENDEPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: The whole EPIN[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepin;
#[doc = "An acknowledged data transfer has taken place on the control endpoint"]
pub struct EVENTS_EP0DATADONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "An acknowledged data transfer has taken place on the control endpoint"]
pub mod events_ep0datadone;
#[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub struct EVENTS_ENDISOIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoin;
#[doc = "Description collection[n]: The whole EPOUT[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub struct EVENTS_ENDEPOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: The whole EPOUT[n] buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepout;
#[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub struct EVENTS_ENDISOOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoout;
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub struct EVENTS_SOF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub mod events_sof;
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub struct EVENTS_USBEVENT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub mod events_usbevent;
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub struct EVENTS_EP0SETUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub mod events_ep0setup;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub struct EVENTS_EPDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub mod events_epdata;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable or disable interrupt"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Details on what caused the USBEVENT event"]
pub struct EVENTCAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Details on what caused the USBEVENT event"]
pub mod eventcause;
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
pub struct EPSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
pub mod epstatus;
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
pub struct EPDATASTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
pub mod epdatastatus;
#[doc = "Device USB address"]
pub struct USBADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device USB address"]
pub mod usbaddr;
#[doc = "SETUP data, byte 0, bmRequestType"]
pub struct BMREQUESTTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 0, bmRequestType"]
pub mod bmrequesttype;
#[doc = "SETUP data, byte 1, bRequest"]
pub struct BREQUEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 1, bRequest"]
pub mod brequest;
#[doc = "SETUP data, byte 2, LSB of wValue"]
pub struct WVALUEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 2, LSB of wValue"]
pub mod wvaluel;
#[doc = "SETUP data, byte 3, MSB of wValue"]
pub struct WVALUEH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 3, MSB of wValue"]
pub mod wvalueh;
#[doc = "SETUP data, byte 4, LSB of wIndex"]
pub struct WINDEXL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 4, LSB of wIndex"]
pub mod windexl;
#[doc = "SETUP data, byte 5, MSB of wIndex"]
pub struct WINDEXH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 5, MSB of wIndex"]
pub mod windexh;
#[doc = "SETUP data, byte 6, LSB of wLength"]
pub struct WLENGTHL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 6, LSB of wLength"]
pub mod wlengthl;
#[doc = "SETUP data, byte 7, MSB of wLength"]
pub struct WLENGTHH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SETUP data, byte 7, MSB of wLength"]
pub mod wlengthh;
#[doc = "Enable USB"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable USB"]
pub mod enable;
#[doc = "Control of the USB pull-up"]
pub struct USBPULLUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control of the USB pull-up"]
pub mod usbpullup;
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
pub struct DPDMVALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
pub mod dpdmvalue;
#[doc = "Data toggle control and status"]
pub struct DTOGGLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data toggle control and status"]
pub mod dtoggle;
#[doc = "Endpoint IN enable"]
pub struct EPINEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint IN enable"]
pub mod epinen;
#[doc = "Endpoint OUT enable"]
pub struct EPOUTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint OUT enable"]
pub mod epouten;
#[doc = "STALL endpoints"]
pub struct EPSTALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "STALL endpoints"]
pub mod epstall;
#[doc = "Controls the split of ISO buffers"]
pub struct ISOSPLIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the split of ISO buffers"]
pub mod isosplit;
#[doc = "Returns the current value of the start of frame counter"]
pub struct FRAMECNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Returns the current value of the start of frame counter"]
pub mod framecntr;
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
pub struct LOWPOWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
pub mod lowpower;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub struct ISOINCONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub mod isoinconfig;
