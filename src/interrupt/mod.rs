use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm ! ( "\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    " ) ;
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm ! ( "\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    " ) ;
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak POWER_CLOCK\nPOWER_CLOCK = DH_TRAMPOLINE\n.weak RADIO\nRADIO = DH_TRAMPOLINE\n.weak UARTE0_UART0\nUARTE0_UART0 = DH_TRAMPOLINE\n.weak SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0\nSPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 = DH_TRAMPOLINE\n.weak SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1\nSPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 = DH_TRAMPOLINE\n.weak NFCT\nNFCT = DH_TRAMPOLINE\n.weak GPIOTE\nGPIOTE = DH_TRAMPOLINE\n.weak SAADC\nSAADC = DH_TRAMPOLINE\n.weak TIMER0\nTIMER0 = DH_TRAMPOLINE\n.weak TIMER1\nTIMER1 = DH_TRAMPOLINE\n.weak TIMER2\nTIMER2 = DH_TRAMPOLINE\n.weak RTC0\nRTC0 = DH_TRAMPOLINE\n.weak TEMP\nTEMP = DH_TRAMPOLINE\n.weak RNG\nRNG = DH_TRAMPOLINE\n.weak ECB\nECB = DH_TRAMPOLINE\n.weak CCM_AAR\nCCM_AAR = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC1\nRTC1 = DH_TRAMPOLINE\n.weak QDEC\nQDEC = DH_TRAMPOLINE\n.weak COMP_LPCOMP\nCOMP_LPCOMP = DH_TRAMPOLINE\n.weak SWI0_EGU0\nSWI0_EGU0 = DH_TRAMPOLINE\n.weak SWI1_EGU1\nSWI1_EGU1 = DH_TRAMPOLINE\n.weak SWI2_EGU2\nSWI2_EGU2 = DH_TRAMPOLINE\n.weak SWI3_EGU3\nSWI3_EGU3 = DH_TRAMPOLINE\n.weak SWI4_EGU4\nSWI4_EGU4 = DH_TRAMPOLINE\n.weak SWI5_EGU5\nSWI5_EGU5 = DH_TRAMPOLINE\n.weak TIMER3\nTIMER3 = DH_TRAMPOLINE\n.weak TIMER4\nTIMER4 = DH_TRAMPOLINE\n.weak PWM0\nPWM0 = DH_TRAMPOLINE\n.weak PDM\nPDM = DH_TRAMPOLINE\n.weak MWU\nMWU = DH_TRAMPOLINE\n.weak PWM1\nPWM1 = DH_TRAMPOLINE\n.weak PWM2\nPWM2 = DH_TRAMPOLINE\n.weak SPIM2_SPIS2_SPI2\nSPIM2_SPIS2_SPI2 = DH_TRAMPOLINE\n.weak RTC2\nRTC2 = DH_TRAMPOLINE\n.weak I2S\nI2S = DH_TRAMPOLINE\n.weak FPU\nFPU = DH_TRAMPOLINE" ) ;
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
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 39] = [
    Some(POWER_CLOCK),
    Some(RADIO),
    Some(UARTE0_UART0),
    Some(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0),
    Some(SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1),
    Some(NFCT),
    Some(GPIOTE),
    Some(SAADC),
    Some(TIMER0),
    Some(TIMER1),
    Some(TIMER2),
    Some(RTC0),
    Some(TEMP),
    Some(RNG),
    Some(ECB),
    Some(CCM_AAR),
    Some(WDT),
    Some(RTC1),
    Some(QDEC),
    Some(COMP_LPCOMP),
    Some(SWI0_EGU0),
    Some(SWI1_EGU1),
    Some(SWI2_EGU2),
    Some(SWI3_EGU3),
    Some(SWI4_EGU4),
    Some(SWI5_EGU5),
    Some(TIMER3),
    Some(TIMER4),
    Some(PWM0),
    Some(PDM),
    None,
    None,
    Some(MWU),
    Some(PWM1),
    Some(PWM2),
    Some(SPIM2_SPIS2_SPI2),
    Some(RTC2),
    Some(I2S),
    Some(FPU),
];
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
unsafe impl Nr for Interrupt {
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
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
