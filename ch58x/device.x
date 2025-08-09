/* Core interrupt sources and trap handlers */
PROVIDE(NonMaskable = DefaultHandler);
PROVIDE(_start_NonMaskable_trap = _start_DefaultHandler_trap);
PROVIDE(Exception = DefaultHandler);
PROVIDE(_start_Exception_trap = _start_DefaultHandler_trap);
PROVIDE(MachineEnvCall = DefaultHandler);
PROVIDE(_start_MachineEnvCall_trap = _start_DefaultHandler_trap);
PROVIDE(UserEnvCall = DefaultHandler);
PROVIDE(_start_UserEnvCall_trap = _start_DefaultHandler_trap);
PROVIDE(Breakpoint = DefaultHandler);
PROVIDE(_start_Breakpoint_trap = _start_DefaultHandler_trap);
PROVIDE(SysTick = DefaultHandler);
PROVIDE(_start_SysTick_trap = _start_DefaultHandler_trap);
PROVIDE(Software = DefaultHandler);
PROVIDE(_start_Software_trap = _start_DefaultHandler_trap);
/* External interrupt sources */
PROVIDE(TMR0 = DefaultHandler);
PROVIDE(GPIOA = DefaultHandler);
PROVIDE(GPIOB = DefaultHandler);
PROVIDE(SPI0 = DefaultHandler);
PROVIDE(BLEB = DefaultHandler);
PROVIDE(BLEL = DefaultHandler);
PROVIDE(USB = DefaultHandler);
PROVIDE(USB2 = DefaultHandler);
PROVIDE(TMR1 = DefaultHandler);
PROVIDE(TMR2 = DefaultHandler);
PROVIDE(UART0 = DefaultHandler);
PROVIDE(UART1 = DefaultHandler);
PROVIDE(RTC = DefaultHandler);
PROVIDE(ADC = DefaultHandler);
PROVIDE(I2C = DefaultHandler);
PROVIDE(PWMx = DefaultHandler);
PROVIDE(TMR3 = DefaultHandler);
PROVIDE(UART2 = DefaultHandler);
PROVIDE(UART3 = DefaultHandler);
PROVIDE(WDOG_BAT = DefaultHandler);

