use stm32f4xx_hal as hal;
use stm32f4xx_hal::pac;

use core::str::from_utf8;

use crate::commands::COMMANDS;
use crate::consoleio::*;
use crate::context::Context;

const MAX_COMMAND_LEN: usize = 64;

static SEP_CHAR: u8 = ' ' as u8;
static CR_CHAR: u8 = '\r' as u8;
static LF_CHAR: u8 = '\n' as u8;

const WELCOME_MESSAGE: &str = r#"
+---------------------------------------------+
| Welcome to the Playroom RGB Controller CLI! |
+---------------------------------------------+
Use command "help" to list available commands
"#;
const CONSOLE_PROMPT: &str = "> ";

pub struct Console {
    buffer: [u8; MAX_COMMAND_LEN],
    write_cursor: usize,
    read_cursor: usize,
}
impl Console {
    pub fn init(ctx: &mut Context) -> Self {

        ctx.io.write(WELCOME_MESSAGE);
        ctx.io.write(CONSOLE_PROMPT);

        Self {
            buffer: [0; MAX_COMMAND_LEN],
            write_cursor: 0,
            read_cursor: 0,
        }
    }

    pub fn process(&mut self, ctx: &mut Context) {
        let received = ctx.io.receive(&mut self.buffer[self.write_cursor..]);
        if received > 0 {
            self.write_cursor += received;
            if let Some(nl) = self.find_newline() {
                let mut argv: [Option<&str>; MAX_COMMAND_LEN] = [None; MAX_COMMAND_LEN];
                let mut argc = 0;
                for (i, c) in self.buffer[0..nl + 1].iter().enumerate() {
                    if *c == SEP_CHAR || *c == CR_CHAR || *c == LF_CHAR {
                        argv[argc] = from_utf8(&self.buffer[self.read_cursor..i]).ok();
                        argc += 1;
                        self.read_cursor = i + 1;
                    }
                }
                if let Some(name) = argv[0] {
                    if name.len() > 0 {
                        let mut found = false;
                        for command in COMMANDS.iter() {
                            if command.name == name {
                                (command.execute)(ctx, &argv);
                                found = true;
                            }
                        }
                        if !found {
                            ctx.io.write("Command not found.\n");
                        }
                    }
                }
                ctx.io.write(CONSOLE_PROMPT);
                self.clear_buffer();
            }
        }
    }

    pub fn find_newline(&self) -> Option<usize> {
        for (i, c) in self.buffer.iter().enumerate() {
            if *c == CR_CHAR {
                return Some(i);
            }
        }
        None
    }

    fn clear_buffer(&mut self) {
        self.buffer = [0; MAX_COMMAND_LEN];
        self.write_cursor = 0;
        self.read_cursor = 0;
    }
}
