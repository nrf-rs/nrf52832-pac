#![doc = "Peripheral access API for NRF52 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UARTE0_UART0();
    fn SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0();
    fn SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1();
    fn NFCT();
    fn GPIOTE();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn COMP_LPCOMP();
    fn SWI0_EGU0();
    fn SWI1_EGU1();
    fn SWI2_EGU2();
    fn SWI3_EGU3();
    fn SWI4_EGU4();
    fn SWI5_EGU5();
    fn TIMER3();
    fn TIMER4();
    fn PWM0();
    fn PDM();
    fn MWU();
    fn PWM1();
    fn PWM2();
    fn SPIM2_SPIS2_SPI2();
    fn RTC2();
    fn I2S();
    fn FPU();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 39] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector {
        _handler: UARTE0_UART0,
    },
    Vector {
        _handler: SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    },
    Vector {
        _handler: SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
    },
    Vector { _handler: NFCT },
    Vector { _handler: GPIOTE },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector {
        _handler: COMP_LPCOMP,
    },
    Vector {
        _handler: SWI0_EGU0,
    },
    Vector {
        _handler: SWI1_EGU1,
    },
    Vector {
        _handler: SWI2_EGU2,
    },
    Vector {
        _handler: SWI3_EGU3,
    },
    Vector {
        _handler: SWI4_EGU4,
    },
    Vector {
        _handler: SWI5_EGU5,
    },
    Vector { _handler: TIMER3 },
    Vector { _handler: TIMER4 },
    Vector { _handler: PWM0 },
    Vector { _handler: PDM },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: MWU },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector {
        _handler: SPIM2_SPIS2_SPI2,
    },
    Vector { _handler: RTC2 },
    Vector { _handler: I2S },
    Vector { _handler: FPU },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK,
    #[doc = "1 - RADIO"]
    RADIO,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0,
    #[doc = "3 - SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0"]
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    #[doc = "4 - SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1"]
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
    #[doc = "5 - NFCT"]
    NFCT,
    #[doc = "6 - GPIOTE"]
    GPIOTE,
    #[doc = "7 - SAADC"]
    SAADC,
    #[doc = "8 - TIMER0"]
    TIMER0,
    #[doc = "9 - TIMER1"]
    TIMER1,
    #[doc = "10 - TIMER2"]
    TIMER2,
    #[doc = "11 - RTC0"]
    RTC0,
    #[doc = "12 - TEMP"]
    TEMP,
    #[doc = "13 - RNG"]
    RNG,
    #[doc = "14 - ECB"]
    ECB,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR,
    #[doc = "16 - WDT"]
    WDT,
    #[doc = "17 - RTC1"]
    RTC1,
    #[doc = "18 - QDEC"]
    QDEC,
    #[doc = "19 - COMP_LPCOMP"]
    COMP_LPCOMP,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1,
    #[doc = "22 - SWI2_EGU2"]
    SWI2_EGU2,
    #[doc = "23 - SWI3_EGU3"]
    SWI3_EGU3,
    #[doc = "24 - SWI4_EGU4"]
    SWI4_EGU4,
    #[doc = "25 - SWI5_EGU5"]
    SWI5_EGU5,
    #[doc = "26 - TIMER3"]
    TIMER3,
    #[doc = "27 - TIMER4"]
    TIMER4,
    #[doc = "28 - PWM0"]
    PWM0,
    #[doc = "29 - PDM"]
    PDM,
    #[doc = "32 - MWU"]
    MWU,
    #[doc = "33 - PWM1"]
    PWM1,
    #[doc = "34 - PWM2"]
    PWM2,
    #[doc = "35 - SPIM2_SPIS2_SPI2"]
    SPIM2_SPIS2_SPI2,
    #[doc = "36 - RTC2"]
    RTC2,
    #[doc = "37 - I2S"]
    I2S,
    #[doc = "38 - FPU"]
    FPU,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::POWER_CLOCK => 0,
            Interrupt::RADIO => 1,
            Interrupt::UARTE0_UART0 => 2,
            Interrupt::SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => 3,
            Interrupt::SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 => 4,
            Interrupt::NFCT => 5,
            Interrupt::GPIOTE => 6,
            Interrupt::SAADC => 7,
            Interrupt::TIMER0 => 8,
            Interrupt::TIMER1 => 9,
            Interrupt::TIMER2 => 10,
            Interrupt::RTC0 => 11,
            Interrupt::TEMP => 12,
            Interrupt::RNG => 13,
            Interrupt::ECB => 14,
            Interrupt::CCM_AAR => 15,
            Interrupt::WDT => 16,
            Interrupt::RTC1 => 17,
            Interrupt::QDEC => 18,
            Interrupt::COMP_LPCOMP => 19,
            Interrupt::SWI0_EGU0 => 20,
            Interrupt::SWI1_EGU1 => 21,
            Interrupt::SWI2_EGU2 => 22,
            Interrupt::SWI3_EGU3 => 23,
            Interrupt::SWI4_EGU4 => 24,
            Interrupt::SWI5_EGU5 => 25,
            Interrupt::TIMER3 => 26,
            Interrupt::TIMER4 => 27,
            Interrupt::PWM0 => 28,
            Interrupt::PDM => 29,
            Interrupt::MWU => 32,
            Interrupt::PWM1 => 33,
            Interrupt::PWM2 => 34,
            Interrupt::SPIM2_SPIS2_SPI2 => 35,
            Interrupt::RTC2 => 36,
            Interrupt::I2S => 37,
            Interrupt::FPU => 38,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Factory Information Configuration Registers"]
pub struct FICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR {}
impl FICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ficr::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for FICR {
    type Target = ficr::RegisterBlock;
    fn deref(&self) -> &ficr::RegisterBlock {
        unsafe { &*FICR::ptr() }
    }
}
#[doc = "Factory Information Configuration Registers"]
pub mod ficr;
#[doc = "User Information Configuration Registers"]
pub struct UICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR {}
impl UICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uicr::RegisterBlock {
        268439552 as *const _
    }
}
impl Deref for UICR {
    type Target = uicr::RegisterBlock;
    fn deref(&self) -> &uicr::RegisterBlock {
        unsafe { &*UICR::ptr() }
    }
}
#[doc = "User Information Configuration Registers"]
pub mod uicr;
#[doc = "Block Protect"]
pub struct BPROT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPROT {}
impl BPROT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bprot::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for BPROT {
    type Target = bprot::RegisterBlock;
    fn deref(&self) -> &bprot::RegisterBlock {
        unsafe { &*BPROT::ptr() }
    }
}
#[doc = "Block Protect"]
pub mod bprot;
#[doc = "Power control"]
pub struct POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER {}
impl POWER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const power::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for POWER {
    type Target = power::RegisterBlock;
    fn deref(&self) -> &power::RegisterBlock {
        unsafe { &*POWER::ptr() }
    }
}
#[doc = "Power control"]
pub mod power;
#[doc = "Clock control"]
pub struct CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK {}
impl CLOCK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clock::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for CLOCK {
    type Target = clock::RegisterBlock;
    fn deref(&self) -> &clock::RegisterBlock {
        unsafe { &*CLOCK::ptr() }
    }
}
#[doc = "Clock control"]
pub mod clock;
#[doc = "2.4 GHz Radio"]
pub struct RADIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO {}
impl RADIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const radio::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for RADIO {
    type Target = radio::RegisterBlock;
    fn deref(&self) -> &radio::RegisterBlock {
        unsafe { &*RADIO::ptr() }
    }
}
#[doc = "2.4 GHz Radio"]
pub mod radio;
#[doc = "UART with EasyDMA"]
pub struct UARTE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0 {}
impl UARTE0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for UARTE0 {
    type Target = uarte0::RegisterBlock;
    fn deref(&self) -> &uarte0::RegisterBlock {
        unsafe { &*UARTE0::ptr() }
    }
}
#[doc = "UART with EasyDMA"]
pub mod uarte0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub struct SPIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0 {}
impl SPIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for SPIM0 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub mod spim0;
#[doc = "SPI Slave 0"]
pub struct SPIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0 {}
impl SPIS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for SPIS0 {
    type Target = spis0::RegisterBlock;
    fn deref(&self) -> &spis0::RegisterBlock {
        unsafe { &*SPIS0::ptr() }
    }
}
#[doc = "SPI Slave 0"]
pub mod spis0;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub struct TWIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0 {}
impl TWIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWIM0 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub mod twim0;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub struct TWIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0 {}
impl TWIS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWIS0 {
    type Target = twis0::RegisterBlock;
    fn deref(&self) -> &twis0::RegisterBlock {
        unsafe { &*TWIS0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub mod twis0;
#[doc = "Serial Peripheral Interface 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0"]
pub mod spi0;
#[doc = "I2C compatible Two-Wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Interface 0"]
pub mod twi0;
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub struct SPIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1 {}
impl SPIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIM1 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM1::ptr() }
    }
}
#[doc = "SPI Slave 1"]
pub struct SPIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1 {}
impl SPIS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIS1 {
    type Target = spis0::RegisterBlock;
    fn deref(&self) -> &spis0::RegisterBlock {
        unsafe { &*SPIS1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub struct TWIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1 {}
impl TWIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TWIM1 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub struct TWIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1 {}
impl TWIS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TWIS1 {
    type Target = twis0::RegisterBlock;
    fn deref(&self) -> &twis0::RegisterBlock {
        unsafe { &*TWIS1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "NFC-A compatible radio"]
pub struct NFCT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NFCT {}
impl NFCT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nfct::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for NFCT {
    type Target = nfct::RegisterBlock;
    fn deref(&self) -> &nfct::RegisterBlock {
        unsafe { &*NFCT::ptr() }
    }
}
#[doc = "NFC-A compatible radio"]
pub mod nfct;
#[doc = "GPIO Tasks and Events"]
pub struct GPIOTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE {}
impl GPIOTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiote::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for GPIOTE {
    type Target = gpiote::RegisterBlock;
    fn deref(&self) -> &gpiote::RegisterBlock {
        unsafe { &*GPIOTE::ptr() }
    }
}
#[doc = "GPIO Tasks and Events"]
pub mod gpiote;
#[doc = "Analog to Digital Converter"]
pub struct SAADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC {}
impl SAADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const saadc::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for SAADC {
    type Target = saadc::RegisterBlock;
    fn deref(&self) -> &saadc::RegisterBlock {
        unsafe { &*SAADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod saadc;
#[doc = "Timer/Counter 0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0;
#[doc = "Timer/Counter 1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub struct RTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0 {}
impl RTC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for RTC0 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC0::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub mod rtc0;
#[doc = "Temperature Sensor"]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const temp::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    fn deref(&self) -> &temp::RegisterBlock {
        unsafe { &*TEMP::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub mod temp;
#[doc = "Random Number Generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1073795072 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator"]
pub mod rng;
#[doc = "AES ECB Mode Encryption"]
pub struct ECB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB {}
impl ECB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecb::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for ECB {
    type Target = ecb::RegisterBlock;
    fn deref(&self) -> &ecb::RegisterBlock {
        unsafe { &*ECB::ptr() }
    }
}
#[doc = "AES ECB Mode Encryption"]
pub mod ecb;
#[doc = "AES CCM Mode Encryption"]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    fn deref(&self) -> &ccm::RegisterBlock {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "AES CCM Mode Encryption"]
pub mod ccm;
#[doc = "Accelerated Address Resolver"]
pub struct AAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR {}
impl AAR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aar::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for AAR {
    type Target = aar::RegisterBlock;
    fn deref(&self) -> &aar::RegisterBlock {
        unsafe { &*AAR::ptr() }
    }
}
#[doc = "Accelerated Address Resolver"]
pub mod aar;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real time counter 1"]
pub struct RTC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1 {}
impl RTC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for RTC1 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC1::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct QDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC {}
impl QDEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qdec::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for QDEC {
    type Target = qdec::RegisterBlock;
    fn deref(&self) -> &qdec::RegisterBlock {
        unsafe { &*QDEC::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub mod qdec;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Low Power Comparator"]
pub struct LPCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP {}
impl LPCOMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpcomp::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for LPCOMP {
    type Target = lpcomp::RegisterBlock;
    fn deref(&self) -> &lpcomp::RegisterBlock {
        unsafe { &*LPCOMP::ptr() }
    }
}
#[doc = "Low Power Comparator"]
pub mod lpcomp;
#[doc = "Software interrupt 0"]
pub struct SWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI0 {}
impl SWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for SWI0 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI0::ptr() }
    }
}
#[doc = "Software interrupt 0"]
pub mod swi0;
#[doc = "Event Generator Unit 0"]
pub struct EGU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0 {}
impl EGU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for EGU0 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU0::ptr() }
    }
}
#[doc = "Event Generator Unit 0"]
pub mod egu0;
#[doc = "Software interrupt 1"]
pub struct SWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI1 {}
impl SWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for SWI1 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI1::ptr() }
    }
}
#[doc = "Event Generator Unit 1"]
pub struct EGU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1 {}
impl EGU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for EGU1 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU1::ptr() }
    }
}
#[doc = "Software interrupt 2"]
pub struct SWI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI2 {}
impl SWI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for SWI2 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI2::ptr() }
    }
}
#[doc = "Event Generator Unit 2"]
pub struct EGU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2 {}
impl EGU2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for EGU2 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU2::ptr() }
    }
}
#[doc = "Software interrupt 3"]
pub struct SWI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI3 {}
impl SWI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073836032 as *const _
    }
}
impl Deref for SWI3 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI3::ptr() }
    }
}
#[doc = "Event Generator Unit 3"]
pub struct EGU3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3 {}
impl EGU3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073836032 as *const _
    }
}
impl Deref for EGU3 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU3::ptr() }
    }
}
#[doc = "Software interrupt 4"]
pub struct SWI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI4 {}
impl SWI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for SWI4 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI4::ptr() }
    }
}
#[doc = "Event Generator Unit 4"]
pub struct EGU4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4 {}
impl EGU4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for EGU4 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU4::ptr() }
    }
}
#[doc = "Software interrupt 5"]
pub struct SWI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI5 {}
impl SWI5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for SWI5 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI5::ptr() }
    }
}
#[doc = "Event Generator Unit 5"]
pub struct EGU5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5 {}
impl EGU5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for EGU5 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU5::ptr() }
    }
}
#[doc = "Timer/Counter 3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073848320 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "Timer/Counter 4"]
pub struct TIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER4 {}
impl TIMER4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073852416 as *const _
    }
}
impl Deref for TIMER4 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER4::ptr() }
    }
}
#[doc = "Pulse Width Modulation Unit 0"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Pulse Width Modulation Unit 0"]
pub mod pwm0;
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub struct PDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM {}
impl PDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm::RegisterBlock {
        1073860608 as *const _
    }
}
impl Deref for PDM {
    type Target = pdm::RegisterBlock;
    fn deref(&self) -> &pdm::RegisterBlock {
        unsafe { &*PDM::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub mod pdm;
#[doc = "Non Volatile Memory Controller"]
pub struct NVMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC {}
impl NVMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmc::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for NVMC {
    type Target = nvmc::RegisterBlock;
    fn deref(&self) -> &nvmc::RegisterBlock {
        unsafe { &*NVMC::ptr() }
    }
}
#[doc = "Non Volatile Memory Controller"]
pub mod nvmc;
#[doc = "Programmable Peripheral Interconnect"]
pub struct PPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPI {}
impl PPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ppi::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for PPI {
    type Target = ppi::RegisterBlock;
    fn deref(&self) -> &ppi::RegisterBlock {
        unsafe { &*PPI::ptr() }
    }
}
#[doc = "Programmable Peripheral Interconnect"]
pub mod ppi;
#[doc = "Memory Watch Unit"]
pub struct MWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MWU {}
impl MWU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mwu::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for MWU {
    type Target = mwu::RegisterBlock;
    fn deref(&self) -> &mwu::RegisterBlock {
        unsafe { &*MWU::ptr() }
    }
}
#[doc = "Memory Watch Unit"]
pub mod mwu;
#[doc = "Pulse Width Modulation Unit 1"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Pulse Width Modulation Unit 2"]
pub struct PWM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2 {}
impl PWM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for PWM2 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM2::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub struct SPIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2 {}
impl SPIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for SPIM2 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM2::ptr() }
    }
}
#[doc = "SPI Slave 2"]
pub struct SPIS2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS2 {}
impl SPIS2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for SPIS2 {
    type Target = spis0::RegisterBlock;
    fn deref(&self) -> &spis0::RegisterBlock {
        unsafe { &*SPIS2::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Real time counter 2"]
pub struct RTC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC2 {}
impl RTC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for RTC2 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC2::ptr() }
    }
}
#[doc = "Inter-IC Sound"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s::RegisterBlock {
        1073893376 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    fn deref(&self) -> &i2s::RegisterBlock {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "Inter-IC Sound"]
pub mod i2s;
#[doc = "GPIO Port 1"]
pub struct P0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0 {}
impl P0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const p0::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for P0 {
    type Target = p0::RegisterBlock;
    fn deref(&self) -> &p0::RegisterBlock {
        unsafe { &*P0::ptr() }
    }
}
#[doc = "GPIO Port 1"]
pub mod p0;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "BPROT"]
    pub BPROT: BPROT,
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UARTE0"]
    pub UARTE0: UARTE0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "SPIM0"]
    pub SPIM0: SPIM0,
    #[doc = "SPIS0"]
    pub SPIS0: SPIS0,
    #[doc = "TWIM0"]
    pub TWIM0: TWIM0,
    #[doc = "TWIS0"]
    pub TWIS0: TWIS0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "SPIM1"]
    pub SPIM1: SPIM1,
    #[doc = "SPIS1"]
    pub SPIS1: SPIS1,
    #[doc = "TWIM1"]
    pub TWIM1: TWIM1,
    #[doc = "TWIS1"]
    pub TWIS1: TWIS1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "NFCT"]
    pub NFCT: NFCT,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "SAADC"]
    pub SAADC: SAADC,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "RTC0"]
    pub RTC0: RTC0,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "ECB"]
    pub ECB: ECB,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "LPCOMP"]
    pub LPCOMP: LPCOMP,
    #[doc = "SWI0"]
    pub SWI0: SWI0,
    #[doc = "EGU0"]
    pub EGU0: EGU0,
    #[doc = "SWI1"]
    pub SWI1: SWI1,
    #[doc = "EGU1"]
    pub EGU1: EGU1,
    #[doc = "SWI2"]
    pub SWI2: SWI2,
    #[doc = "EGU2"]
    pub EGU2: EGU2,
    #[doc = "SWI3"]
    pub SWI3: SWI3,
    #[doc = "EGU3"]
    pub EGU3: EGU3,
    #[doc = "SWI4"]
    pub SWI4: SWI4,
    #[doc = "EGU4"]
    pub EGU4: EGU4,
    #[doc = "SWI5"]
    pub SWI5: SWI5,
    #[doc = "EGU5"]
    pub EGU5: EGU5,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: TIMER4,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PDM"]
    pub PDM: PDM,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
    #[doc = "MWU"]
    pub MWU: MWU,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "PWM2"]
    pub PWM2: PWM2,
    #[doc = "SPIM2"]
    pub SPIM2: SPIM2,
    #[doc = "SPIS2"]
    pub SPIS2: SPIS2,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "RTC2"]
    pub RTC2: RTC2,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "P0"]
    pub P0: P0,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            BPROT: BPROT {
                _marker: PhantomData,
            },
            POWER: POWER {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UARTE0: UARTE0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            SPIM0: SPIM0 {
                _marker: PhantomData,
            },
            SPIS0: SPIS0 {
                _marker: PhantomData,
            },
            TWIM0: TWIM0 {
                _marker: PhantomData,
            },
            TWIS0: TWIS0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            SPIM1: SPIM1 {
                _marker: PhantomData,
            },
            SPIS1: SPIS1 {
                _marker: PhantomData,
            },
            TWIM1: TWIM1 {
                _marker: PhantomData,
            },
            TWIS1: TWIS1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            NFCT: NFCT {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            SAADC: SAADC {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            RTC0: RTC0 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            ECB: ECB {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            AAR: AAR {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC1: RTC1 {
                _marker: PhantomData,
            },
            QDEC: QDEC {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            LPCOMP: LPCOMP {
                _marker: PhantomData,
            },
            SWI0: SWI0 {
                _marker: PhantomData,
            },
            EGU0: EGU0 {
                _marker: PhantomData,
            },
            SWI1: SWI1 {
                _marker: PhantomData,
            },
            EGU1: EGU1 {
                _marker: PhantomData,
            },
            SWI2: SWI2 {
                _marker: PhantomData,
            },
            EGU2: EGU2 {
                _marker: PhantomData,
            },
            SWI3: SWI3 {
                _marker: PhantomData,
            },
            EGU3: EGU3 {
                _marker: PhantomData,
            },
            SWI4: SWI4 {
                _marker: PhantomData,
            },
            EGU4: EGU4 {
                _marker: PhantomData,
            },
            SWI5: SWI5 {
                _marker: PhantomData,
            },
            EGU5: EGU5 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            TIMER4: TIMER4 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            PDM: PDM {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
            MWU: MWU {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            PWM2: PWM2 {
                _marker: PhantomData,
            },
            SPIM2: SPIM2 {
                _marker: PhantomData,
            },
            SPIS2: SPIS2 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            RTC2: RTC2 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            P0: P0 {
                _marker: PhantomData,
            },
        }
    }
}
