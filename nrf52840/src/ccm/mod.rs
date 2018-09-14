#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encryption/decryption"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    pub tasks_rateoverride: TASKS_RATEOVERRIDE,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Key-stream generation complete"]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt complete"]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - Deprecated register - CCM error event"]
    pub events_error: EVENTS_ERROR,
    _reserved7: [u8; 244usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - MIC check result"]
    pub micstatus: MICSTATUS,
    _reserved11: [u8; 252usize],
    #[doc = "0x500 - Enable"]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode"]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to data structure holding AES key and NONCE vector"]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Input pointer"]
    pub inptr: INPTR,
    #[doc = "0x510 - Output pointer"]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
    #[doc = "0x518 - Length of key-stream generated when MODE.LENGTH = Extended."]
    pub maxpacketsize: MAXPACKETSIZE,
    #[doc = "0x51c - Data rate override setting."]
    pub rateoverride: RATEOVERRIDE,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub struct TASKS_KSGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub struct TASKS_CRYPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "Stop encryption/decryption"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub struct TASKS_RATEOVERRIDE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub mod tasks_rateoverride;
#[doc = "Key-stream generation complete"]
pub struct EVENTS_ENDKSGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key-stream generation complete"]
pub mod events_endksgen;
#[doc = "Encrypt/decrypt complete"]
pub struct EVENTS_ENDCRYPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "Deprecated register - CCM error event"]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - CCM error event"]
pub mod events_error;
#[doc = "Shortcut register"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "MIC check result"]
pub struct MICSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "Enable"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable"]
pub mod enable;
#[doc = "Operation mode"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operation mode"]
pub mod mode;
#[doc = "Pointer to data structure holding AES key and NONCE vector"]
pub struct CNFPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to data structure holding AES key and NONCE vector"]
pub mod cnfptr;
#[doc = "Input pointer"]
pub struct INPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "Output pointer"]
pub struct OUTPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "Pointer to data area used for temporary storage"]
pub struct SCRATCHPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
#[doc = "Length of key-stream generated when MODE.LENGTH = Extended."]
pub struct MAXPACKETSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length of key-stream generated when MODE.LENGTH = Extended."]
pub mod maxpacketsize;
#[doc = "Data rate override setting."]
pub struct RATEOVERRIDE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data rate override setting."]
pub mod rateoverride;
