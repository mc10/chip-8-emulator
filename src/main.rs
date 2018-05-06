extern crate failure;
extern crate piston_window;
extern crate rand;

mod cpu;
mod graphics;
mod keypad;
mod opcode;

use std::fs::File;
use std::io::Read;

use failure::Error;

use cpu::Cpu;

const DEFAULT_ROM: &str = "PONG";

fn main() {
    ::std::process::exit(match app() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    })
}

fn app() -> Result<(), Error> {
    let mut rom_buf = Vec::new();
    read_rom(DEFAULT_ROM, &mut rom_buf)?;

    let mut cpu = Cpu::new(&rom_buf)?;
    cpu.cycle();
    Ok(())
}

fn read_rom(name: &str, buf: &mut Vec<u8>) -> Result<(), Error> {
    let mut f = File::open(format!("roms/{}.rom", name))?;
    f.read_to_end(buf)?;
    Ok(())
}
