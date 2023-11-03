use crate::consoleio::*;

const VERSION_STRING: &str = "playroom-rgbctl 0.1.0";

pub const COMMANDS: [Command; 2] = [
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
];

#[repr(C)]
pub struct Command {
    pub name: &'static str,
    pub execute: fn(&mut ConsoleIO, &[Option<&str>]),
    pub help: &'static str,
}

fn help(io: &mut ConsoleIO, _argv: &[Option<&str>]) {
    io.write("Available commands:\n\n");
    for command in COMMANDS {
        io.write(command.name);
        io.write(" -- ");
        io.write(command.help);
        io.write("\n");
    }
}

fn ver(io: &mut ConsoleIO, _argv: &[Option<&str>]) {
    io.write(VERSION_STRING);
    io.write("\n");
}
