/*
Driver to connect a Saitek Radio to Flightgear flight simulator

MIT License

Copyright (c) 2023 Dave Attwood

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

 */

//!
//!   This program links a Saitek Radio Panel to FlightGear Flight  Simulator.
//!   The code runs (without any changes) on Linux and Windows.


use radio_device::Device;
use simulator::Simulator;
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

fn run(input: String, output: String) {
    let mut my_device = Device::new();
    let mut my_sim = Simulator::new(input, output);
    loop {
        my_sim.read();
        my_device.read();

        if my_device.has_control_input_updated() {
            let (command_option, value) = my_device.make_command(); // can return Option(None) which is no write required
            if let Some(radio) = command_option { my_sim.write(radio, value) }
        }

        if my_sim.has_input_updated() || my_device.has_selection_input_updated() {
            my_device.update_display(&my_sim);
        }
        my_sim.preserve_current_input();
        my_device.preserve_current_input();
    }
}
