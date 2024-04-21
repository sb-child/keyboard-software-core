#![no_std]
#![no_main]

use core;

use cortex_m::singleton;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use kbd_core::{bootloader, design, rgb};
use panic_probe as _;
use pio_proc::pio_file;
use rp2040_hal::{
  adc,
  clocks::init_clocks_and_plls,
  dma::{single_buffer, DMAExt},
  gpio,
  multicore::{Multicore, Stack},
  pac, pio,
  pio::PIOExt,
  Clock, Sio, Watchdog,
};

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

static mut CORE1_STACK: Stack<4096> = Stack::new();

fn core1_task() {
  info!("hello from core 1");
  loop {}
}

#[entry]
fn main() -> ! {
  info!("启动");
  let mut pac = pac::Peripherals::take().unwrap();
  let core = pac::CorePeripherals::take().unwrap();
  let mut watchdog = Watchdog::new(pac.WATCHDOG);
  let mut sio = Sio::new(pac.SIO);

  // External high-speed crystal on the pico board is 12Mhz
  let external_xtal_freq_hz = 12_000_000u32;
  let clocks = init_clocks_and_plls(
    external_xtal_freq_hz,
    pac.XOSC,
    pac.CLOCKS,
    pac.PLL_SYS,
    pac.PLL_USB,
    &mut pac.RESETS,
    &mut watchdog,
  )
  .ok()
  .unwrap();

  // core 1
  {
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
    let cores = mc.cores();
    let core1 = &mut cores[1];
    let _ = core1.spawn(unsafe { &mut CORE1_STACK.mem }, core1_task).unwrap();
  }

  let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
  let pins = gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
  let mut dma = pac.DMA.split(&mut pac.RESETS);

  let mut led_pin = pins.gpio26.into_push_pull_output();

  led_pin.set_high().unwrap();

  // PIO0
  let (mut pio0, pio0_sm0, pio0_sm1, pio0_sm2, pio0_sm3) = pac.PIO0.split(&mut pac.RESETS);
  let _pio0_led0: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio27.into_function();
  let _pio0_led1: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio28.into_function();
  let _pio0_led2: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio29.into_function();
  let _pio0_m2_in1: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio6.into_function();
  let _pio0_m2_in2: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio7.into_function();
  let _pio0_m2_in3: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio8.into_function();
  let _pio0_m2_in4: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio9.into_function();
  let _pio0_m2_in5: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio10.into_function();
  let _pio0_m2_in6: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio11.into_function();
  let _pio0_m2_out1: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio12.into_function();
  let _pio0_m2_out2: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio13.into_function();
  let _pio0_m2_out3: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio14.into_function();
  let _pio0_m2_out4: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio15.into_function();
  let _pio0_m2_out5: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio16.into_function();
  let _pio0_m2_out6: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio17.into_function();
  let _pio0_m2_out7: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio18.into_function();
  let _pio0_m2_out8: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio19.into_function();
  let pio0_prog_led = pio_file!("src/pio-asm/leds.pio", select_program("led_ctrl"),);
  let pio0_prog_kbd = pio_file!("src/pio-asm/leds.pio", select_program("kbd_scan"),);
  let pio0_installed_sm0 = pio0.install(&pio0_prog_led.program).unwrap();
  let pio0_installed_sm1 = pio0.install(&pio0_prog_led.program).unwrap();
  let pio0_installed_sm2 = pio0.install(&pio0_prog_led.program).unwrap();
  let pio0_installed_sm3 = pio0.install(&pio0_prog_kbd.program).unwrap();
  pio0.irq0().enable_sm_interrupt(3);
  // led ctrl (1)
  let (mut pio0_sm0, _, mut pio0_sm0_tx) = pio::PIOBuilder::from_installed_program(pio0_installed_sm0)
    .set_pins(27, 1) // _pio0_led0
    .autopull(true)
    .pull_threshold(24)
    .in_shift_direction(pio::ShiftDirection::Left)
    .out_shift_direction(pio::ShiftDirection::Left)
    .buffers(pio::Buffers::OnlyTx)
    .side_set_pin_base(27) // _pio0_led0
    .clock_divisor_fixed_point(15, 160) // 125 MHz / 15.625 = 0.125 us
    .build(pio0_sm0);
  // led ctrl (2)
  let (mut pio0_sm1, _, mut pio0_sm1_tx) = pio::PIOBuilder::from_installed_program(pio0_installed_sm1)
    .set_pins(28, 1) // _pio0_led1
    .autopull(true)
    .pull_threshold(24)
    .in_shift_direction(pio::ShiftDirection::Left)
    .out_shift_direction(pio::ShiftDirection::Left)
    .buffers(pio::Buffers::OnlyTx)
    .side_set_pin_base(28) // _pio0_led1
    .clock_divisor_fixed_point(15, 160)
    .build(pio0_sm1);
  // led ctrl (3)
  let (mut pio0_sm2, _, mut pio0_sm2_tx) = pio::PIOBuilder::from_installed_program(pio0_installed_sm2)
    .set_pins(29, 1) // _pio0_led2
    .autopull(true)
    .pull_threshold(24)
    .in_shift_direction(pio::ShiftDirection::Left)
    .out_shift_direction(pio::ShiftDirection::Left)
    .buffers(pio::Buffers::OnlyTx)
    .side_set_pin_base(29) // _pio0_led2
    .clock_divisor_fixed_point(15, 160)
    .build(pio0_sm2);
  // kbd scan (2)
  let (mut pio0_sm3, mut pio0_sm3_rx, _) = pio::PIOBuilder::from_installed_program(pio0_installed_sm3)
  .in_pin_base(12) // M2_OUT_1
  .set_pins(6, 5) // M2_IN_1
  .side_set_pin_base(11) // M2_IN_6
  .out_pins(6, 6) // M2_IN_1
  .autopush(true)
  .push_threshold(32)
  .buffers(pio::Buffers::OnlyRx)
  .clock_divisor_fixed_point(62, 128) // 125 MHz / 62.5 = 0.5 us
  .in_shift_direction(pio::ShiftDirection::Left)
  .out_shift_direction(pio::ShiftDirection::Left)
  .build(pio0_sm3);
  pio0_sm0.set_pindirs([(27, pio::PinDir::Output)]);
  pio0_sm1.set_pindirs([(28, pio::PinDir::Output)]);
  pio0_sm2.set_pindirs([(29, pio::PinDir::Output)]);
  pio0_sm3.set_pindirs([
    (6, pio::PinDir::Output),
    (7, pio::PinDir::Output),
    (8, pio::PinDir::Output),
    (9, pio::PinDir::Output),
    (10, pio::PinDir::Output),
    (11, pio::PinDir::Output),
    (12, pio::PinDir::Input),
    (13, pio::PinDir::Input),
    (14, pio::PinDir::Input),
    (15, pio::PinDir::Input),
    (16, pio::PinDir::Input),
    (17, pio::PinDir::Input),
    (18, pio::PinDir::Input),
    (19, pio::PinDir::Input),
  ]);
  pio0_sm0.start();
  pio0_sm1.start();
  pio0_sm2.start();
  pio0_sm3.start();
  let mut pio0_led_line1_dma_buf = singleton!(: [u32; 36] = [0; 36]).unwrap();
  let mut pio0_led_line2_dma_buf = singleton!(: [u32; 48] = [0; 48]).unwrap();
  let mut pio0_led_line3_dma_buf = singleton!(: [u32; 26] = [0; 26]).unwrap();

  // PIO1
  let (mut pio1, pio1_sm0, pio1_sm1, pio1_sm2, pio1_sm3) = pac.PIO1.split(&mut pac.RESETS);
  let _pio1_tm1637_data: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio2.into_function();
  let _pio1_tm1637_clk: gpio::Pin<_, gpio::FunctionPio0, _> = pins.gpio3.into_function();
  let pio1_prog_tm1637 = pio_file!("src/pio-asm/tm1637.pio", select_program("tm1637"),);
  let pio1_installed_sm0 = pio1.install(&pio1_prog_tm1637.program).unwrap();
  // tm1637
  let (mut pio1_sm0, _, mut pio1_sm0_tx) = pio::PIOBuilder::from_installed_program(pio1_installed_sm0)
  .set_pins(2, 1) // _pio1_tm1637_data
  .out_pins(2, 1) // _pio1_tm1637_data
  // .autopull(true)
  .pull_threshold(32)
  .out_shift_direction(pio::ShiftDirection::Right)
  .buffers(pio::Buffers::OnlyTx)
  .side_set_pin_base(3) // _pio1_tm1637_clk
  .clock_divisor_fixed_point(2800, 128) // 125 MHz / 2800.5 = 22.404 us
  .build(pio1_sm0);
  pio1_sm0.set_pindirs([(2, pio::PinDir::Output), (3, pio::PinDir::Output)]);
  pio1_sm0.set_pins([(2, pio::PinState::Low), (3, pio::PinState::Low)]);
  pio1_sm0.start();
  // FIXME: this won't work because of hardware issue
  pio1_sm0_tx.write(0xffc0448a);
  pio1_sm0_tx.write(0xffc1448a);
  pio1_sm0_tx.write(0xffc2448a);

  led_pin.set_high().unwrap();

  let mut x = 0.5f32;
  let mut dir = true;
  let y = 0.005f32;
  loop {
    if dir {
      x += y;
    } else {
      x -= y;
    }
    if x < 0.2f32 {
      x = 0.2f32;
      dir = true;
      led_pin.set_high().unwrap();
    }
    if x > 1f32 {
      x = 1f32;
      dir = false;
      led_pin.set_low().unwrap();
    }
    let alpha = x; // (255.0 * x) as u8;
    pio0.clear_irq(0b00000001); // 清除中断, 触发键盘扫描

    {
      // 设置初值
      for x in &mut *pio0_led_line1_dma_buf {
        *x = rgb::rgba(255, 20, 20, alpha);
      }
      for x in &mut *pio0_led_line2_dma_buf {
        *x = rgb::rgba(20, 255, 20, alpha);
      }
      for x in &mut *pio0_led_line3_dma_buf {
        *x = rgb::rgba(20, 20, 255, alpha);
      }
      // 启动dma
      let pio0_led_line1_dma = single_buffer::Config::new(dma.ch0, pio0_led_line1_dma_buf, pio0_sm0_tx).start();
      let pio0_led_line2_dma = single_buffer::Config::new(dma.ch1, pio0_led_line2_dma_buf, pio0_sm1_tx).start();
      let pio0_led_line3_dma = single_buffer::Config::new(dma.ch2, pio0_led_line3_dma_buf, pio0_sm2_tx).start();
      // 去忙别的...
      (dma.ch0, pio0_led_line1_dma_buf, pio0_sm0_tx) = pio0_led_line1_dma.wait();
      (dma.ch1, pio0_led_line2_dma_buf, pio0_sm1_tx) = pio0_led_line2_dma.wait();
      (dma.ch2, pio0_led_line3_dma_buf, pio0_sm2_tx) = pio0_led_line3_dma.wait();
    }

    // for _ in 0..5 {
    //   pio0_sm0_tx.write(rgb::rgba(255, 10, 10, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(10, 255, 10, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(10, 10, 255, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(255, 255, 255, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(20, 30, 255, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(255, 255, 255, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm0_tx.write(rgb::rgba(20, 30, 255, alpha));

    //   pio0_sm1_tx.write(rgb::rgba(255, 10, 10, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(10, 255, 10, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(10, 10, 255, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));

    //   pio0_sm1_tx.write(rgb::rgba(20, 30, 255, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm1_tx.write(rgb::rgba(20, 30, 255, alpha));
    //   while !pio0_sm0_tx.is_empty() {}
    //   while !pio0_sm1_tx.is_empty() {}
    // }
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 10, alpha));
    // pio0_sm1_tx.write(rgb::rgba(10, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 10, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // while !pio0_sm1_tx.is_empty() {}
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm1_tx.write(rgb::rgba(255, 255, 255, alpha));
    // while !pio0_sm1_tx.is_empty() {}
    // for _ in 0..3 {
    //   pio0_sm2_tx.write(rgb::rgba(255, 10, 10, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(10, 255, 10, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(10, 10, 255, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(255, 255, 255, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(255, 255, 255, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(255, 10, 200, alpha));
    //   pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    //   while !pio0_sm2_tx.is_empty() {}
    // }
    // pio0_sm2_tx.write(rgb::rgba(255, 255, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(255, 10, 200, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // pio0_sm2_tx.write(rgb::rgba(20, 30, 255, alpha));
    // while !pio0_sm2_tx.is_empty() {}

    // delay.delay_ms(1); // 去忙别的...
    pio0_sm3_rx.read(); // 读出32位的结果
    pio0_sm3_rx.read(); // 再读出32位
    delay.delay_ms(5);
  }

  // pio0_sm3_rx.read();

  // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
  // on-board LED, it might need to be changed.
  // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
  // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
  // LED to one of the GPIO pins, and reference that pin here.

  let mut adc = adc::Adc::new(pac.ADC, &mut pac.RESETS);
  let mut temp_sensor = adc.take_temp_sensor().unwrap();
  let mut temp_fifo = adc.build_fifo().clock_divider(47999, 0).set_channel(&mut temp_sensor).start_paused();
  temp_fifo.resume();
  let step: f32 = 0.0008058608;
  loop {
    let mut val = (temp_fifo.read_single() as f32) * step;

    led_pin.set_high().unwrap();
    for i in 0..512 {
      val *= 1.12345678;
    }
    delay.delay_ms(500);
    // delay.delay_ms(250);
    led_pin.set_low().unwrap();
    for i in 0..512 {
      val /= 1.12345678;
    }
    delay.delay_ms(250);
    let temp = 27.0 - (val - 0.706) / 0.001721;
    info!("温度: {}°C", temp);
    // bootloader::into_usbboot();
  }
}

// End of file
