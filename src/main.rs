use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use crc_all::Crc;
use rustyline::{Editor, error::ReadlineError};
use serialport::SerialPort;

fn crc8_maxim(bytes: &[u8]) -> u8 {
    Crc::<u8>::new(0x31, 8, 0x00, 0x00, true).update(bytes)
}

enum BrewieCommand {
    P80(f32, (), f32, f32),
    P110,
    P111,
    P112,
    P113,
    P114,
    P115,
    P116,
    P117,
    P118,
    P119,
    P120,
    P121,
    P122,
    P123,
    P124,
    P125,
    P126,
    P127,
    P128,
    P129,
    P130,
    P131,
    P132,
    P133,
    P134,
    P135,
    P136,
    P137,
    P150(u16),
    P151(u16),
    P205(bool),
}

impl BrewieCommand {
    fn to_string(&self) -> String {
        match self {
            BrewieCommand::P80(to_liter, _, mash_temperature_delta, boil_temperature_delta) => format!("P80 {:.1} {} {:.5} {:.5}", to_liter, 0, mash_temperature_delta, boil_temperature_delta),
            BrewieCommand::P110 => String::from("P110"),
            BrewieCommand::P111 => String::from("P111"),
            BrewieCommand::P112 => String::from("P112"),
            BrewieCommand::P113 => String::from("P113"),
            BrewieCommand::P114 => String::from("P114"),
            BrewieCommand::P115 => String::from("P115"),
            BrewieCommand::P116 => String::from("P116"),
            BrewieCommand::P117 => String::from("P117"),
            BrewieCommand::P118 => String::from("P118"),
            BrewieCommand::P119 => String::from("P119"),
            BrewieCommand::P120 => String::from("P120"),
            BrewieCommand::P121 => String::from("P121"),
            BrewieCommand::P122 => String::from("P122"),
            BrewieCommand::P123 => String::from("P123"),
            BrewieCommand::P124 => String::from("P124"),
            BrewieCommand::P125 => String::from("P125"),
            BrewieCommand::P126 => String::from("P126"),
            BrewieCommand::P127 => String::from("P127"),
            BrewieCommand::P128 => String::from("P128"),
            BrewieCommand::P129 => String::from("P129"),
            BrewieCommand::P130 => String::from("P130"),
            BrewieCommand::P131 => String::from("P131"),
            BrewieCommand::P132 => String::from("P132"),
            BrewieCommand::P133 => String::from("P133"),
            BrewieCommand::P134 => String::from("P134"),
            BrewieCommand::P135 => String::from("P135"),
            BrewieCommand::P136 => String::from("P136"),
            BrewieCommand::P137 => String::from("P137"),
            BrewieCommand::P150(value) => format!("P150 {}", value),
            BrewieCommand::P151(value) => format!("P151 {}", value),
            BrewieCommand::P205(value) => format!("P205 {}", *value as u8),
        }
    }

    fn serialize(&self, n: u8) -> Vec<u8> {
        let cmd = self.to_string();
        let crc = crc8_maxim(cmd.as_bytes());

        let mut result = Vec::with_capacity(5 + cmd.len());

        result.push(0x24);
        result.push(n);

        result.push(cmd.len() as u8);
        result.extend(cmd.as_bytes());
        result.push(crc);
        result.push(0x2A);

        result
    }
}

struct Board {
    n: u8,
    port: Box<dyn SerialPort>,
}

impl Board {
    fn open<'a>(path: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        Self { n: 0, port: serialport::new(path, 115200).open().unwrap() }
    }

    fn reset(&self) {
        let mut file = File::create("/dev/brewie-mcu-reset/value").unwrap();
        file.write_all(b"0\n").unwrap();
        sleep(Duration::from_micros(100));
        file.write_all(b"1\n").unwrap();
        sleep(Duration::from_micros(100));
    }

    fn send_cmd(&mut self, cmd: &BrewieCommand) -> std::io::Result<()> {
        self.n = self.n.wrapping_add(1);
        self.port.write_all(&cmd.serialize(self.n))
    }
}

fn main() {
    let mut board = Board::open("/dev/ttyS1");
    let mut editor = Editor::<()>::new();

    println!("Resetting board");
    board.reset();
    sleep(Duration::from_secs(4));

    println!("Initializing board");
    board.send_cmd(&BrewieCommand::P80(16059.1, (), 1.51759, 1.50194)).unwrap();
    sleep(Duration::from_secs(4));

    loop {
        let line = match editor.readline("> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        };

        editor.add_history_entry(&line);

        let result = match line.as_str() {
            "open water inlet" => board.send_cmd(&BrewieCommand::P110),
            "close water inlet" => board.send_cmd(&BrewieCommand::P111),
            "open mash inlet" => board.send_cmd(&BrewieCommand::P112),
            "close mash inlet" => board.send_cmd(&BrewieCommand::P113),
            "open boil inlet" => board.send_cmd(&BrewieCommand::P114),
            "close boil inlet" => board.send_cmd(&BrewieCommand::P115),
            "open hop 1" => board.send_cmd(&BrewieCommand::P116),
            "close hop 1" => board.send_cmd(&BrewieCommand::P117),
            "open hop 2" => board.send_cmd(&BrewieCommand::P118),
            "close hop 2" => board.send_cmd(&BrewieCommand::P119),
            "open hop 3" => board.send_cmd(&BrewieCommand::P120),
            "close hop 3" => board.send_cmd(&BrewieCommand::P121),
            "open hop 4" => board.send_cmd(&BrewieCommand::P122),
            "close hop 4" => board.send_cmd(&BrewieCommand::P123),
            "start mash pump" => board.send_cmd(&BrewieCommand::P124),
            "stop mash pump" => board.send_cmd(&BrewieCommand::P125),
            "start boil pump" => board.send_cmd(&BrewieCommand::P126),
            "stop boil pump" => board.send_cmd(&BrewieCommand::P127),
            "open cool inlet" => board.send_cmd(&BrewieCommand::P128),
            "close cool inlet" => board.send_cmd(&BrewieCommand::P129),
            "open cool valve" => board.send_cmd(&BrewieCommand::P130),
            "close cool valve" => board.send_cmd(&BrewieCommand::P131),
            "open outlet valve" => board.send_cmd(&BrewieCommand::P132),
            "close outlet valve" => board.send_cmd(&BrewieCommand::P133),
            "open mash return" => board.send_cmd(&BrewieCommand::P134),
            "close mash return" => board.send_cmd(&BrewieCommand::P135),
            "open boil return" => board.send_cmd(&BrewieCommand::P136),
            "close boil return" => board.send_cmd(&BrewieCommand::P137),
            line => { println!("Unknown command: {}", line); continue }
        };

        match result {
            Ok(_) => println!("ok"),
            Err(err) => println!("error: {}", err),
        }
    }
}
