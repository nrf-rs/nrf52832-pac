#![doc = "Peripheral access API for NRF51 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UART0();
    fn SPI0_TWI0();
    fn SPI1_TWI1();
    fn GPIOTE();
    fn ADC();
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
    fn LPCOMP();
    fn SWI0();
    fn SWI1();
    fn SWI2();
    fn SWI3();
    fn SWI4();
    fn SWI5();
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
pub static __INTERRUPTS: [Vector; 26] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector { _handler: UART0 },
    Vector {
        _handler: SPI0_TWI0,
    },
    Vector {
        _handler: SPI1_TWI1,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE },
    Vector { _handler: ADC },
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
    Vector { _handler: LPCOMP },
    Vector { _handler: SWI0 },
    Vector { _handler: SWI1 },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
    Vector { _handler: SWI4 },
    Vector { _handler: SWI5 },
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
    #[doc = "2 - UART0"]
    UART0,
    #[doc = "3 - SPI0_TWI0"]
    SPI0_TWI0,
    #[doc = "4 - SPI1_TWI1"]
    SPI1_TWI1,
    #[doc = "6 - GPIOTE"]
    GPIOTE,
    #[doc = "7 - ADC"]
    ADC,
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
    #[doc = "19 - LPCOMP"]
    LPCOMP,
    #[doc = "20 - SWI0"]
    SWI0,
    #[doc = "21 - SWI1"]
    SWI1,
    #[doc = "22 - SWI2"]
    SWI2,
    #[doc = "23 - SWI3"]
    SWI3,
    #[doc = "24 - SWI4"]
    SWI4,
    #[doc = "25 - SWI5"]
    SWI5,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::POWER_CLOCK => 0,
            Interrupt::RADIO => 1,
            Interrupt::UART0 => 2,
            Interrupt::SPI0_TWI0 => 3,
            Interrupt::SPI1_TWI1 => 4,
            Interrupt::GPIOTE => 6,
            Interrupt::ADC => 7,
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
            Interrupt::LPCOMP => 19,
            Interrupt::SWI0 => 20,
            Interrupt::SWI1 => 21,
            Interrupt::SWI2 => 22,
            Interrupt::SWI3 => 23,
            Interrupt::SWI4 => 24,
            Interrupt::SWI5 => 25,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Power Control."]
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
#[doc = "Power Control."]
pub mod power;
#[doc = "Clock control."]
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
#[doc = "Clock control."]
pub mod clock;
#[doc = "The radio."]
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
#[doc = "The radio."]
pub mod radio;
#[doc = "Universal Asynchronous Receiver/Transmitter."]
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
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub mod uart0;
#[doc = "SPI master 0."]
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
#[doc = "SPI master 0."]
pub mod spi0;
#[doc = "Two-wire interface master 0."]
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
#[doc = "Two-wire interface master 0."]
pub mod twi0;
#[doc = "SPI master 1."]
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
#[doc = "Two-wire interface master 1."]
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
#[doc = "SPI slave 1."]
pub struct SPIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1 {}
impl SPIS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis1::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIS1 {
    type Target = spis1::RegisterBlock;
    fn deref(&self) -> &spis1::RegisterBlock {
        unsafe { &*SPIS1::ptr() }
    }
}
#[doc = "SPI slave 1."]
pub mod spis1;
#[doc = "GPIO tasks and events."]
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
#[doc = "GPIO tasks and events."]
pub mod gpiote;
#[doc = "Analog to digital converter."]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog to digital converter."]
pub mod adc;
#[doc = "Timer 0."]
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
#[doc = "Timer 0."]
pub mod timer0;
#[doc = "Timer 1."]
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
#[doc = "Timer 2."]
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
#[doc = "Real time counter 0."]
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
#[doc = "Real time counter 0."]
pub mod rtc0;
#[doc = "Temperature Sensor."]
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
#[doc = "Temperature Sensor."]
pub mod temp;
#[doc = "Random Number Generator."]
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
#[doc = "Random Number Generator."]
pub mod rng;
#[doc = "AES ECB Mode Encryption."]
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
#[doc = "AES ECB Mode Encryption."]
pub mod ecb;
#[doc = "Accelerated Address Resolver."]
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
#[doc = "Accelerated Address Resolver."]
pub mod aar;
#[doc = "AES CCM Mode Encryption."]
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
#[doc = "AES CCM Mode Encryption."]
pub mod ccm;
#[doc = "Watchdog Timer."]
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
#[doc = "Watchdog Timer."]
pub mod wdt;
#[doc = "Real time counter 1."]
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
#[doc = "Rotary decoder."]
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
#[doc = "Rotary decoder."]
pub mod qdec;
#[doc = "Low power comparator."]
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
#[doc = "Low power comparator."]
pub mod lpcomp;
#[doc = "SW Interrupts."]
pub struct SWI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI {}
impl SWI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for SWI {
    type Target = swi::RegisterBlock;
    fn deref(&self) -> &swi::RegisterBlock {
        unsafe { &*SWI::ptr() }
    }
}
#[doc = "SW Interrupts."]
pub mod swi;
#[doc = "Non Volatile Memory Controller."]
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
#[doc = "Non Volatile Memory Controller."]
pub mod nvmc;
#[doc = "PPI controller."]
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
#[doc = "PPI controller."]
pub mod ppi;
#[doc = "Factory Information Configuration."]
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
#[doc = "Factory Information Configuration."]
pub mod ficr;
#[doc = "User Information Configuration."]
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
#[doc = "User Information Configuration."]
pub mod uicr;
#[doc = "General purpose input and output."]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General purpose input and output."]
pub mod gpio;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "SPIS1"]
    pub SPIS1: SPIS1,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "ADC"]
    pub ADC: ADC,
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
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "LPCOMP"]
    pub LPCOMP: LPCOMP,
    #[doc = "SWI"]
    pub SWI: SWI,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
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
            POWER: POWER {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            SPIS1: SPIS1 {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            ADC: ADC {
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
            AAR: AAR {
                _marker: PhantomData,
            },
            CCM: CCM {
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
            LPCOMP: LPCOMP {
                _marker: PhantomData,
            },
            SWI: SWI {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
        }
    }
}
