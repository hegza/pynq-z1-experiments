use common::Mmio;

// Addresses as retrieved in Python from `pynq.PL.ip_dict`, from base overlay MMIO at /dev/mem using sudo
const LED_ADDRESS: u32 = 0x41240000; // `rgbleds_gpio`, C_BASEADDR = 0x41240000
const LED_RANGE: u32 = 0x10000; // 65536, addr_range
const LED_OFFSET: usize = 0; // GPIO_DATA

fn main() {
    let mut leds = Mmio::map(LED_ADDRESS, LED_RANGE);

    let red = 0b100;
    let blue = 0b001;
    let black = 0b000;
    for _ in 0..10 {
        leds[LED_OFFSET] = (red << 3) | black;
        std::thread::sleep(std::time::Duration::from_millis(200));
        leds[LED_OFFSET] = (black << 3) | blue;
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    leds[0] = (black << 3) | black;
}
