use crate::context::Context;

const VERSION_STRING: &str = "playroom-rgbctl 0.1.0";

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

fn rgb(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write("Missing implementation.");
    ctx.io.write("\n");
}

fn imu(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write("Missing implementation.");
    ctx.io.write("\n");
}

fn battery(ctx: &mut Context, _argv: &[Option<&str>]) {
    ctx.io.write("Missing implementation.");
    ctx.io.write("\n");
}
