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

use std::net::UdpSocket;
use std::process;
use std::str;

// names the simulator responds to, for the radio_device selector switch position
const COMMAND: [&str; 7] = [
    "comm",
    "comm[1]",
    "nav",
    "nav[1]",
    "adf",
    "dme",
    "transponder",
];

pub struct Simulator {
    input_address: String,
    output_address: String,
    input_current: String,
    input_old: String,
}

impl Simulator {
    /// Reply a Simulator object to access the FGFS data stream
    pub fn new(input: String, output: String) -> Simulator {
        Simulator {
            input_address: input,
            output_address: output,
            input_current: String::new(),
            input_old: String::new(),
        }
    }

    /// Perform a blocking read of FGFS output (defined by 'saitekradio.xml')
    /// The incoming data is 14 CSV items
    pub fn read(&mut self) {
        {
            let socket =
                UdpSocket::bind(&self.input_address).expect("Socket create incoming error");

            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 120];
            let amt = socket.recv(&mut buf).expect("Socket receive error");
            self.input_current = str::from_utf8(&buf[..amt]).unwrap().trim().to_string();
            // self.radios = self.input_current.split(',').collect();
        }
    }

    /// Send a command to the FGFS consisting of the simulator name for the radio number
    /// input, and the action (which is one of 0, 1, 2, 4, 8)
    pub fn write(&self, radionum: usize, action: u8) {
        let data = format!("{},{}\n", COMMAND[radionum], action);
        // println!("Writing {}", data);
        let buf = data.into_bytes();
        // Following required to avoid getting 'address in use' error
        // Copied from https://illegalargumentexception.blogspot.com/2015/05/rust-send-and-receive-on-localhost-with.html
        let socket = UdpSocket::bind(&self.input_address).expect("Socket create incoming error");
        socket
            .send_to(&buf, &self.output_address)
            .expect("Socket send error");
    }

    /// Saves the current input as the previous input
    pub fn preserve_current_input(&mut self) {
        self.input_old = self.input_current.clone();
    }

    ///  Returns true if the current input differs from the previous input
    pub fn has_input_updated(&self) -> bool {
        self.input_current != self.input_old
    }

    /// Returns a tuple containing the active and standby radios as Strings
    pub fn get_radio_data(&self, radio: usize) -> (String, String) {
        let data: Vec<&str> = self.input_current.split(',').collect();
        if data.len() != 14 {
            println!(
                "Bad read from simulator, {:} items received, expected 14\n\
            Perhaps incorrect file specified on FGFS output port 60001 on startup?",
                data.len()
            );
            process::exit(2);
        }
        (data[radio * 2].to_string(), data[radio * 2 + 1].to_string())
    }
}
