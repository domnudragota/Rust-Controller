#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::str::from_utf8;

use byte_slice_cast::AsByteSlice;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_futures::select;
use embassy_net::tcp::TcpSocket;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{Config, IpAddress, IpEndpoint, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
use heapless::Vec;
use log::{info, warn};
use static_cell::StaticCell;
use embassy_rp::adc::{Adc, Channel, Config as AdcConfig, InterruptHandler as AdcInterruptHandler};


// USB driver
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, Endpoint, InterruptHandler as USBInterruptHandler};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    ADC_IRQ_FIFO => AdcInterruptHandler;
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &str = "dragota_a";
const WIFI_PASSWORD: &str = "sygd6347";

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut adc = Adc::new(peripherals.ADC, Irqs, AdcConfig::default());
    // Start the USB logger driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    let mut button_a = Input::new(peripherals.PIN_6, Pull::Up); //xbox
    let mut button_lt =  Input::new(peripherals.PIN_9, Pull::Up); // LT
    let mut button_rt = Input::new(peripherals.PIN_10, Pull::Up); // RT
    let mut button_lb = Input::new(peripherals.PIN_26, Pull::Up); // LB
    let mut button_rb = Input::new(peripherals.PIN_27, Pull::Up); // RB
    let mut button_back = Input::new(peripherals.PIN_7, Pull::Up); // BACK
    let mut start = Input::new(peripherals.PIN_11, Pull::Up); // START
    let mut button_abxy = Input::new(peripherals.PIN_22, Pull::Up); // ABXY
    let mut button_udlr = Input::new(peripherals.PIN_14, Pull::Up); // UPDOWNLEFTRIGHT


    //JOSYSTICKS

    struct Joystick {
        lx: i16,
        ly: i16,
        rx: i16,
        ry: i16,
     }

     let mut joystick1 = Joystick { lx: 0, ly: 0, rx: 0, ry: 0 };
     let mut joystick2 = Joystick { lx: 0, ly: 0, rx: 0, ry: 0 };

//dont work but this is how i would have implemented it with adc
//     let joystick1 = Joystick {
//         lx_channel: adc.bind_channel(peripherals.GPIO26),
//         ly_channel: adc.bind_channel(peripherals.GPIO27),
//         rx_channel: adc.bind_channel(peripherals.GPIO28),
//         ry_channel: adc.bind_channel(peripherals.GPIO29),
//     };
 async fn update_joystick1(joystick: &mut Joystick) {
         joystick.lx = (joystick.lx + 100).clamp(-32768, 32767);
        joystick.ly = (joystick.ly + 100).clamp(-18976, 18976);
        joystick.rx = (joystick.rx - 100).clamp(-32768, 32767);
       joystick.ry = (joystick.ry - 100).clamp(-18976, 18976);
    }

    async fn update_joystick2(joystick: &mut Joystick) {
         joystick.lx = (joystick.lx + 100).clamp(-32768, 32767);
         joystick.ly = (joystick.ly + 100).clamp(-18976, 18976);
         joystick.rx = (joystick.rx - 100).clamp(-32768, 32767);
         joystick.ry = (joystick.ry - 100).clamp(-18976, 18976);
     }

    // async fn update_joystick(joystick: &mut Joystick, adc: &mut Adc) {
    //     let lx = adc.read(joystick.lx_channel).await.unwrap_or_default();
    //     let ly = adc.read(joystick.ly_channel).await.unwrap_or_default();
    //     let rx = adc.read(joystick.rx_channel).await.unwrap_or_default();
    //     let ry = adc.read(joystick.ry_channel).await.unwrap_or_default();
    //
    //     // Convert ADC reading (0-4095) to joystick range (-32768 to 32767)
    //     joystick.lx = ((lx as i32 * 65535 / 4095) - 32768).clamp(-32768, 32767);
    //     joystick.ly = ((ly as i32 * 65535 / 4095) - 32768).clamp(-32768, 32767);
    //     joystick.rx = ((rx as i32 * 65535 / 4095) - 32768).clamp(-32768, 32767);
    //     joystick.ry = ((ry as i32 * 65535 / 4095) - 32768).clamp(-32768, 32767);
    // }





    let fw = include_bytes!("C:/Users/Andrei/Desktop/lab-main/cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("C:/Users/Andrei/Desktop/lab-main/cyw43-firmware/43439A0_clm.bin");


    let pwr = Output::new(peripherals.PIN_23, Level::Low);
    let cs = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    );


    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();


    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = Config::dhcpv4(Default::default());


    let seed = 0x0123_4567_89ab_cdef;


    static STACK: StaticCell<Stack<cyw43::NetDriver<'static>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<2>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        net_device,
        config,
        RESOURCES.init(StackResources::<2>::new()),
        seed,
    ));


    spawner.spawn(net_task(stack)).unwrap();

    loop {


        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status {}", err.status);
            }
        }
    }

    // Wait for DHCP (not necessary when using static IP)
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up {:?}!", stack.config_v4());

    // And now we can use it!

    // TODO 1: Create buffers
    let mut rx_buffer = [0; 4096];
    let mut rx_metadata_buffer = [PacketMetadata::EMPTY; 3];
    let mut tx_buffer = [0; 4096];
    let mut tx_metadata_buffer = [PacketMetadata::EMPTY; 3];

    let mut buf = [0u8; 4096];

    // TODO 2: Initialize UDP socket
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_metadata_buffer,
        &mut rx_buffer,
        &mut tx_metadata_buffer,
        &mut tx_buffer,
    );



    info!("Starting server on UDP:1234...");


    if let Err(e) = socket.bind(1234) {
        warn!("accept error: {:?}", e);
        return;
    }

    loop {




        let mut packet = [0u8; 30];

        packet[0] = button_a.is_low().into();
        packet[1] = button_lt.is_low().into();
        packet[2] = button_rt.is_low().into();
        packet[3] = button_lb.is_low().into();
        packet[4] = button_rb.is_low().into();
        packet[5] = button_back.is_low().into();
        packet[6] = start.is_low().into();
        packet[7] = button_abxy.is_low().into();
        packet[8] = button_udlr.is_low().into();

        //joystick data

        packet[9..11].clone_from_slice(&joystick1.lx.to_ne_bytes());
        packet[11..13].clone_from_slice(&joystick1.ly.to_ne_bytes());
        packet[13..15].clone_from_slice(&joystick1.rx.to_ne_bytes());
        packet[15..17].clone_from_slice(&joystick1.ry.to_ne_bytes());

        packet[19..21].clone_from_slice(&joystick2.lx.to_ne_bytes());
        packet[21..23].clone_from_slice(&joystick2.ly.to_ne_bytes());
        packet[23..25].clone_from_slice(&joystick2.rx.to_ne_bytes());
        packet[25..27].clone_from_slice(&joystick2.ry.to_ne_bytes());

         update_joystick1(&mut joystick1).await;
         update_joystick2(&mut joystick2).await;


        control.gpio_set(0, true).await;

        match socket
            .send_to(
                &packet,
                IpEndpoint::new(IpAddress::v4(192, 168, 243, 162), 1234), //dynamic, always change it
            )
            .await
        {
            Ok(()) => {
                info!("sent")
            }
            Err(e) => {
                warn!("send error: {:?}", e);
            }
        }
        info!("Sending packet to UDP server...");

        //match for joystick
        match socket.send_to(&packet, IpEndpoint::new(IpAddress::v4(192, 168, 243, 162), 1234)).await {
            Ok(()) => info!("Joystick data sent"),
            Err(e) => warn!("send error: {:?}", e),
         }

    }

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        info!("panic occurred in file '{}' at line {}", location.file(), location.line());
    } else {
        info!("panic occurred but can't get location information...");
    }
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        info!("panic payload: {}", s);
    }
    loop {}
}
