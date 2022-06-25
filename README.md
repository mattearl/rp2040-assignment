# rp2040-assignment

The code in this repository implements the game SmallBall on an
[Adafruit Feather RP2040](https://www.adafruit.com/product/4884) 
connected to an [SSD1306](https://www.adafruit.com/product/938) 
OLED display and an [MPU6050](https://www.adafruit.com/product/3886) 
6-axis IMU via [STEMMA QT cables](https://www.adafruit.com/product/4399). 
The pitch and roll measurements from the IMU are the game control inputs. 

SmallBall is a game where you control a small ball on a small screen via an 
IMU sensor. The goal is to move the ball around the screen to visit all goals 
on the screen in the minimum amount of time. The game keeps track of the
lowest score achieved.

## Running the game on the RP2040 
1.) Prepare the hardware by connecting the display and the IMU to the RP2040 via STEMMA QT cables.

<p align="center"><img src="https://www.dropbox.com/s/m3pdzs1j7k5qpui/PXL_20220624_194844668.MP.jpg?raw=1" alt="system startup" width="600"></p>

2.) Install the RP2040 rust target 
```
rustup target add thumbv6m-none-eabi
```

3.) Install elf2uf2-rs, which is used to deploy the firmware to the RP2040 by converting the
elf file produced by the rust build to uf2 format and then copying the uf2 file to the hardware.
```
cargo install elf2uf2-rs
```

4.) Clone this repository
```
git clone https://github.com/mattearl/rp2040-assignment.git
cd rp2040-assignment
``` 

5.) Boot the board into bootloader mode by holding down the BOOTSEL button while plugging the 
board into the USB port (the board will appear as a USB disk drive).

6.) Build the firmware and flash the hardware
```
cargo run --release
``` 

7.) You will see the game splash screen. After a short wait the game will start and you can play 
by rolling and pitching the IMU to move the ball around the screen to visit each goal as quickly
as possible. Once all goals are reached you will see the game over screen that shows your score
and the lowest score achieved. After a short wait the game will start again. Here is a
[video](https://www.dropbox.com/s/spphcini2hiejfz/PXL_20220624_201321347~2.mp4?raw=1)
of game play.

## Troubleshooting

If you start the system and you see nothing on the display you may have an SSD1306 display with 
a different I2C address than the address on the SSD1306 that I am using. If so, go to `main.rs` 
and replace the line
```rust
let interface = I2CDisplayInterface::new_alternate_address(bus.acquire_i2c());
```
with this line 
```rust
let interface = I2CDisplayInterface::new(bus.acquire_i2c());
```
and try again. If that doesn't work, look up the address in the SSD1306 datasheet and set it 
manually using the `new_custom_address` function.
```rust
let interface = I2CDisplayInterface::new_custom_address(bus.acquire_i2c(), 0x3D);
```
 
## References

- [adafruit-feather-rp2040 hal](https://crates.io/crates/adafruit-feather-rp2040)
- [rp2040-hal](https://crates.io/crates/rp2040-hal)
- [SSD1306 display driver](https://crates.io/crates/ssd1306)
- [MPU6050 6-axis IMU driver](https://crates.io/crates/mpu6050)
- [shared-bus](https://crates.io/crates/shared-bus)
- [heapless](https://crates.io/crates/heapless)


