pub fn into_usbboot() -> ! {
  // gpio26 as usb storage status led
  rp2040_hal::rom_data::reset_to_usb_boot(1 << 26, 0);
  // In case the reboot fails
  loop {
    cortex_m::asm::nop();
  }
}
