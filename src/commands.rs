use numtoa::NumToA;
use core::ops::DerefMut;
use crate::context::Context;
use stm32f4xx_hal::prelude::*;
use crate::rgbcontroller::*;

const VERSION_STRING: &str = "playroom-rgbctl 0.1.0";

const EXIT_CHAR: u8 = 'q' as u8;

pub const COMMANDS: [Command; 6] = [
    Command {
        name: "help",
        execute: help,
        help: "List the available commands",
    },
    Command {
        name: "version",
        execute: ver,
        help: "Display version information",
    },
    Command {
        name: "led",
        execute: led,
        help: r#"Set debug led on/off
        USAGE: led <on|off>"#,
    },
    Command {
        name: "rgb",
        execute: rgb,
        help: "Set and get RGB values",
    },
    Command {
        name: "imu",
        execute: imu,
        help: "Get current IMU readings",
    },
    Command {
        name: "battery",
        execute: battery,
        help: "Display current battery level",
    },
];

#[repr(C)]
pub struct Command {
    pub name: &'static str,
    pub execute: fn(&mut Context, &[Option<&str>]),
    pub help: &'static str,
}

fn help(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write("Available commands:\n\n");
    for command in COMMANDS {
        ctx.io.write(command.name);
        ctx.io.write(" -- ");
        ctx.io.write(command.help);
        ctx.io.write("\n");
    }
}

fn ver(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write(VERSION_STRING);
    ctx.io.write("\n");
}

fn led(ctx: &mut Context, argv: &[Option<&str>]) {
    if let Some(state) = argv[1] {
        match state {
            "on" => {
                ctx.io.write("Setting LED ON.\n");
                ctx.debug_led.set_low()
            }
            "off" => {
                ctx.io.write("Setting LED OFF.\n");
                ctx.debug_led.set_high()
            }
            _ => ctx.io.write("Invalid parameter.\n"),
        }
    }
}

fn rgb(ctx: &mut Context, argv: &[Option<&str>]) {
    cortex_m::interrupt::free(|cs| {
        if let Some(rgb) = RGB.borrow(cs).borrow_mut().deref_mut() {
            if let Some(subcommand) = argv[1] {
                match subcommand {
                    "on" => rgb.on(),
                    "off" => rgb.off(),
                    "set" => {
                        if let (Some(r), Some(g), Some(b)) = (
                            argv[2].and_then(|v| v.parse::<u8>().ok()),
                            argv[3].and_then(|v| v.parse::<u8>().ok()),
                            argv[4].and_then(|v| v.parse::<u8>().ok()),
                        ){
                           rgb.set_color(&Color::Red, r);
                           rgb.set_color(&Color::Green, g);
                           rgb.set_color(&Color::Blue, b);
                        } else {
                            ctx.io.write("Invalid value.\n");
                        }
                    }
                    _ => ctx.io.write("Invalid subcommand.\n"),
                }
            }
        }
    });
}

fn imu(ctx: &mut Context, _argv: &[Option<&str>]) {
    let mut rcvbuf = [0u8];
    let display_freq = 500.millis();
    ctx.counter.start(display_freq).unwrap();
    while rcvbuf[0] != EXIT_CHAR {
        if let Ok(_) = ctx.counter.wait() {
            match ctx.imu.orientation() {
                Ok((p, y, r)) => {
                    let mut buffer = [0u8; 20];
                    ctx.io.write("Pitch: ");
                    ctx.io.write(p.numtoa_str(10, &mut buffer));
                    ctx.io.write("\n");
                    ctx.io.write("Yaw: ");
                    ctx.io.write(y.numtoa_str(10, &mut buffer));
                    ctx.io.write("\n");
                    ctx.io.write("Roll: ");
                    ctx.io.write(r.numtoa_str(10, &mut buffer));
                    ctx.io.write("\n--------\n");
                }
                Err(_) => {
                    ctx.io.write("Error reading from IMU.\n");
                }
            };
            ctx.counter.start(display_freq).unwrap();
        }
        ctx.io.receive(&mut rcvbuf);
    }
}

fn battery(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write("Missing implementation.");
    ctx.io.write("\n");
}
