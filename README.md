# rp2040-assignment
Firmware demonstration for an [Adafruit Feather RP2040](https://www.adafruit.com/product/4884) connected to 
an [SSD1306](https://www.adafruit.com/product/938) OLED display and an 
[MPU6050](https://www.adafruit.com/product/3886) 6-axis IMU via 
[STEMMA QT cables](https://www.adafruit.com/product/4399).

## Running the firmware

1.) Install the RP2040 rust target 
```
rustup target add thumbv6m-none-eabi
```

2.) Install elf2uf2-rs, which is used to depoly the firmware to the RP2040 by converting the
elf file produced by the rust build to uf2 format and then copying the uf2 file to the hardware.
```
cargo install elf2uf2-rs
```

3.) Clone this repository
```
git clone https://github.com/mattearl/rp2040-assignment.git
cd rp2040-assignment
``` 

4.) Boot the board into bootloader mode by holding down the BOOTSEL button while plugging the 
board into the USB port (the board will appear as a USB disk drive).

5.) Build the firmware and flash the hardware
```
cargo run --release
``` 

6.) The dispaly shoudl show graphics and you can use the mpu to provide input.

## Troubleshooting

If you start the system and you see nothing on the display you may have an SSD1306 display with 
a different I2C address than the address on the SSD1306 that I am using. If so, go to `main.rs` 
and replace the line
```
let interface = I2CDisplayInterface::new_alternate_address(bus.acquire_i2c());
```
with this line 
```
let interface = I2CDisplayInterface::new(bus.acquire_i2c());
```
and try again. If that doesn't work, look up the address in the SSD1306 datasheet and set it 
manually using the `new_custom_address` function.
```
let interface = I2CDisplayInterface::new_custom_address(bus.acquire_i2c(), 0x3D);
```
 
## References

- [adafruit-feather-rp2040 hal](https://crates.io/crates/adafruit-feather-rp2040)
- [rp2040-hal](https://crates.io/crates/rp2040-hal)
- [SSD1306 display driver](https://crates.io/crates/ssd1306)
- [MPU6050 6-axis IMU driver](https://crates.io/crates/mpu6050)
- [shared-bus](https://crates.io/crates/shared-bus)
- [heapless](https://crates.io/crates/heapless)


