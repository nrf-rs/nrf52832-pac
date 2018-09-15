#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encrypt/decrypt. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encrypt/decrypt."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - Keystream generation completed."]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt completed."]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - Error happened."]
    pub events_error: EVENTS_ERROR,
    _reserved6: [u8; 244usize],
    #[doc = "0x200 - Shortcuts for the CCM."]
    pub shorts: SHORTS,
    _reserved7: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 244usize],
    #[doc = "0x400 - CCM RX MIC check result."]
    pub micstatus: MICSTATUS,
    _reserved10: [u8; 252usize],
    #[doc = "0x500 - CCM enable."]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode."]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to a data structure holding AES key and NONCE vector."]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Pointer to the input packet."]
    pub inptr: INPTR,
    #[doc = "0x510 - Pointer to the output packet."]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
    pub scratchptr: SCRATCHPTR,
    _reserved16: [u8; 2788usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub struct TASKS_KSGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed."]
pub struct TASKS_CRYPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "Stop encrypt/decrypt."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop encrypt/decrypt."]
pub mod tasks_stop;
#[doc = "Keystream generation completed."]
pub struct EVENTS_ENDKSGEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Keystream generation completed."]
pub mod events_endksgen;
#[doc = "Encrypt/decrypt completed."]
pub struct EVENTS_ENDCRYPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encrypt/decrypt completed."]
pub mod events_endcrypt;
#[doc = "Error happened."]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error happened."]
pub mod events_error;
#[doc = "Shortcuts for the CCM."]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts for the CCM."]
pub mod shorts;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "CCM RX MIC check result."]
pub struct MICSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM RX MIC check result."]
pub mod micstatus;
#[doc = "CCM enable."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CCM enable."]
pub mod enable;
#[doc = "Operation mode."]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Operation mode."]
pub mod mode;
#[doc = "Pointer to a data structure holding AES key and NONCE vector."]
pub struct CNFPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to a data structure holding AES key and NONCE vector."]
pub mod cnfptr;
#[doc = "Pointer to the input packet."]
pub struct INPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to the input packet."]
pub mod inptr;
#[doc = "Pointer to the output packet."]
pub struct OUTPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to the output packet."]
pub mod outptr;
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
pub struct SCRATCHPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
pub mod scratchptr;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
