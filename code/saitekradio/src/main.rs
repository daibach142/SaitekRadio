/*
Driver to connect a Saitek Radio to Flightgear flight simulator
Copyright (C) 2021 Dave Attwood

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

 */

//!
//!   This program links a Saitek Radio Panel to FlightGear Flight  Simulator.
//!   The code runs (without any changes) on Linux and Windows.


use crate::radio_device::Device;
use crate::simulator::Simulator;
use std::env;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    // default addresses

    let output = match args.next() {
        Some(arg) => arg + ":60002",
        None => "127.0.0.1:60002".to_string(),
    };

    let input = match args.next() {
        Some(arg) => arg + ":60001",
        None => "127.0.0.1:60001".to_string(),
    };
    println!("{} Version {} Output {}, Input {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), output, input);
    run(input, output);
    Ok(())
}

mod radio_constants;
mod radio_device;
mod simulator;

fn run(input: String, output: String) {
    let mut my_device = Device::new();
    let mut my_sim = Simulator::new(input, output);
    loop {
        my_sim.read();
        my_device.read();

        if my_device.has_control_input_updated() {
            let (command_option, value) = my_device.make_command();
            match command_option {
                Some(radio) => my_sim.write(radio, value),
                None => (),
            }
        }

        if my_sim.has_input_updated() || my_device.has_selection_input_updated() {
            my_device.update_display(&my_sim);
        }
        my_sim.preserve_current_input();
        my_device.preserve_current_input();
    }
}
