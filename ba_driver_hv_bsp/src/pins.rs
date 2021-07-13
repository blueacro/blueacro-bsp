//! Grand Central M4 Express Pins

use super::{hal, pac::MCLK, target_device};

use hal::define_pins;
use hal::gpio::{self, *};
use hal::clock::GenericClockController;

#[cfg(feature = "usb")]
use super::pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    pin sw0 = b23,
    pin led1 = b10,
    pin led2 = b11,
    pin led3 = b12,

    pin fault = a12,

    pin dim_en = b14,
    pin dim = b15,

    pin vbat = b3,

    // USB
    pin vbus_detecion = a2,
    pin usb_id = a16, // unused
    pin usb_dm = a24,
    pin usb_dp = a25,

    pin swd = a31,
    pin swc = a30,
    pin swo = b30,

    // CAN
    pin can_tx = a22,
    pin can_rx = a23,
    pin can_standby = b22,
);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        let led = LED {
            led_green: self.led1,
            led_red: self.led2,
            led_blue: self.led3,
        };

        Sets {
            port: self.port,
            usb,
            led: led,
            sw0: self.sw0,
            dim: self.dim,
            dim_en: self.dim_en,
            fault: self.fault,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    pub led: LED,
    pub sw0: Pb23<Input<Floating>>,

    /// USB pins
    pub usb: USB,

    /// DIM pin
    pub dim: Pb15<Input<Floating>>,
    pub dim_en: Pb14<Input<Floating>>,

    /// Fault pin
    pub fault: Pa12<Input<Floating>>,

    /// Port
    pub port: Port,
}

pub struct LED {
    pub led_green: Pb10<Input<Floating>>,
    pub led_red: Pb11<Input<Floating>>,
    pub led_blue: Pb12<Input<Floating>>,
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

impl USB {
    #[cfg(feature = "usb")]
    /// Convenience for setting up the onboard usb port to operate
    /// as a USB device.
    pub fn init(
        self,
        usb: super::pac::USB,
        clocks: &mut GenericClockController,
        mclk: &mut MCLK,
        port: &mut Port,
    ) -> UsbBusAllocator<UsbBus> {
        clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
        let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
        let usb_clock = &clocks.usb(&usb_gclk).unwrap();

        UsbBusAllocator::new(UsbBus::new(
            usb_clock,
            mclk,
            self.dm,
            self.dp,
            usb,
        ))
    }
}
