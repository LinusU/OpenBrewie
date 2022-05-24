use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;

use crc_all::Crc;
use serialport::SerialPort;
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut board = Board::open("/dev/ttyS1");
    // let mut board = Board::open("/dev/cu.usbmodem11101");

    // println!("Resetting board");
    // board.reset();
    // sleep(Duration::from_secs(4));

    // println!("Initializing board");
    // board.send_cmd(&BrewieCommand::P80(16059.1, (), 1.51759, 1.50194)).unwrap();
    // sleep(Duration::from_secs(4));

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        // let size = f.size();
        // let block = Block::default()
        //     .title("Block")
        //     .borders(Borders::ALL);
        // f.render_widget(block, size);


        let rows = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ].as_ref()
            )
            .split(f.size());

        let boil = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                ].as_ref()
            )
            .split(rows[0]);

        let block = Paragraph::new("Boil pump")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, boil[0]);

        let block = Paragraph::new("Boil return")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, boil[1]);

        let block = Paragraph::new("Boil inlet")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, boil[2]);

        let mash = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                ].as_ref()
            )
            .split(rows[1]);

        let block = Paragraph::new("Mash pump")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, mash[0]);

        let block = Paragraph::new("Mash return")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, mash[1]);

        let block = Paragraph::new("Mash inlet")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, mash[2]);

        let hops = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ].as_ref()
            )
            .split(rows[2]);

        let block = Paragraph::new("Hop 1")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, hops[0]);

        let block = Paragraph::new("Hop 2")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, hops[1]);

        let block = Paragraph::new("Hop 3")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, hops[2]);

        let block = Paragraph::new("Hop 4")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, hops[3]);

        let valve = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(rows[3]);

        let block = Paragraph::new("Outlet valve")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, valve[0]);

        let block = Paragraph::new("Cool valve")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, valve[1]);

        let inlet = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(rows[4]);

        let block = Paragraph::new("Water inlet")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, inlet[0]);

        let block = Paragraph::new("Cool inlet")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, inlet[1]);

        let heater = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(rows[5]);

        let block = Paragraph::new("Boil heater")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, heater[0]);

        let block = Paragraph::new("Mash heater")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(block, heater[1]);

        let block = Paragraph::new(format!("Water level: {}l", 0));
        f.render_widget(block, rows[6]);

        let block = Paragraph::new(format!("Mash temp: {:.1}ºC", 24.5));
        f.render_widget(block, rows[7]);

        let block = Paragraph::new(format!("Boil temp: {:.1}ºC", 24.6));
        f.render_widget(block, rows[8]);

        // let block = Block::default()
        //     .title("Boil pump")
        //     .borders(Borders::ALL);
        // f.render_widget(block, boil[0]);

        // let block = Block::default()
        //     .title("Boil return")
        //     .borders(Borders::ALL);
        // f.render_widget(block, boil[1]);

        // let block = Block::default()
        //     .title("Boil inlet")
        //     .borders(Borders::ALL);
        // f.render_widget(block, boil[2]);

        // let block = Block::default()
        //     .title("Block")
        //     .borders(Borders::ALL);
        // f.render_widget(block, chunks[0]);
        // let block = Block::default()
        //     .title("Block 2")
        //     .borders(Borders::ALL);
        // f.render_widget(block, chunks[1]);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;



    // loop {
    //     let line = match editor.readline("> ") {
    //         Ok(line) => line,
    //         Err(ReadlineError::Interrupted) => continue,
    //         Err(ReadlineError::Eof) => break,
    //         Err(err) => panic!("{}", err),
    //     };

    //     editor.add_history_entry(&line);

    //     let result = match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
    //         ["open", "water", "inlet"] => board.send_cmd(&BrewieCommand::P110),
    //         ["close", "water", "inlet"] => board.send_cmd(&BrewieCommand::P111),
    //         ["open", "mash", "inlet"] => board.send_cmd(&BrewieCommand::P112),
    //         ["close", "mash", "inlet"] => board.send_cmd(&BrewieCommand::P113),
    //         ["open", "boil", "inlet"] => board.send_cmd(&BrewieCommand::P114),
    //         ["close", "boil", "inlet"] => board.send_cmd(&BrewieCommand::P115),

    //         ["open", "hop", "1"] => board.send_cmd(&BrewieCommand::P116),
    //         ["close", "hop", "1"] => board.send_cmd(&BrewieCommand::P117),
    //         ["open", "hop", "2"] => board.send_cmd(&BrewieCommand::P118),
    //         ["close", "hop", "2"] => board.send_cmd(&BrewieCommand::P119),
    //         ["open", "hop", "3"] => board.send_cmd(&BrewieCommand::P120),
    //         ["close", "hop", "3"] => board.send_cmd(&BrewieCommand::P121),
    //         ["open", "hop", "4"] => board.send_cmd(&BrewieCommand::P122),
    //         ["close", "hop", "4"] => board.send_cmd(&BrewieCommand::P123),

    //         ["start", "mash", "pump"] => board.send_cmd(&BrewieCommand::P124),
    //         ["stop", "mash", "pump"] => board.send_cmd(&BrewieCommand::P125),
    //         ["start", "boil", "pump"] => board.send_cmd(&BrewieCommand::P126),
    //         ["stop", "boil", "pump"] => board.send_cmd(&BrewieCommand::P127),

    //         ["open", "cool", "inlet"] => board.send_cmd(&BrewieCommand::P128),
    //         ["close", "cool", "inlet"] => board.send_cmd(&BrewieCommand::P129),
    //         ["open", "cool", "valve"] => board.send_cmd(&BrewieCommand::P130),
    //         ["close", "cool", "valve"] => board.send_cmd(&BrewieCommand::P131),
    //         ["open", "outlet", "valve"] => board.send_cmd(&BrewieCommand::P132),
    //         ["close", "outlet", "valve"] => board.send_cmd(&BrewieCommand::P133),

    //         ["open", "mash", "return"] => board.send_cmd(&BrewieCommand::P134),
    //         ["close", "mash", "return"] => board.send_cmd(&BrewieCommand::P135),
    //         ["open", "boil", "return"] => board.send_cmd(&BrewieCommand::P136),
    //         ["close", "boil", "return"] => board.send_cmd(&BrewieCommand::P137),

    //         ["exit", "dev", "mode"] => board.send_cmd(&BrewieCommand::P205(false)),
    //         ["enter", "dev", "mode"] => board.send_cmd(&BrewieCommand::P205(true)),

    //         ["set", "mash", "heater", temp_str] => {
    //             let temp: u16 = match temp_str.parse() {
    //                 Ok(temp) => temp,
    //                 Err(_) => { println!("Invalid temperature: {}", temp_str); continue }
    //             };

    //             board.send_cmd(&BrewieCommand::P150(temp))
    //         },

    //         ["set", "boil", "heater", temp_str] => {
    //             let temp: u16 = match temp_str.parse() {
    //                 Ok(temp) => temp,
    //                 Err(_) => { println!("Invalid temperature: {}", temp_str); continue }
    //             };

    //             board.send_cmd(&BrewieCommand::P151(temp))
    //         },

    //         ["open", "hop", cage, ..] => { println!("Unknown hop cage: {}", cage); continue },
    //         ["close", "hop", cage, ..] => { println!("Unknown hop cage: {}", cage); continue },

    //         ["open", valve, ..] => { println!("Unknown valve: {}", valve); continue },
    //         ["close", valve, ..] => { println!("Unknown valve: {}", valve); continue },

    //         ["start", pump, ..] => { println!("Unknown pump: {}", pump); continue },
    //         ["stop", pump, ..] => { println!("Unknown pump: {}", pump); continue },

    //         ["set", heater, ..] => { println!("Unknown heater: {}", heater); continue },

    //         _ => { println!("Unknown command: {}", line); continue }
    //     };

    //     match result {
    //         Ok(_) => println!("ok"),
    //         Err(err) => println!("error: {}", err),
    //     }
    // }

    Ok(())
}
