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
use crate::radio_constants::*;
use crate::simulator::Simulator;
use hidapi::{HidApi, HidDevice};
use std::process;

const VENDOR_ID: u16 = 0x06a3;
const RADIO_ID: u16 = 0x0d05;

const RIGHT_SIZE: usize = 23; // 2 bytes at end unused, required on Windows hidapi

/* The Saitek Radio Device consists of;
   2 rows (TOP and BOTTOM) of
       1 Active 5 digit display
       1 Standby 5 digit display
       1 seven position selector switch (LHS)
       2 rotary knobs (RHS)
       1 ACT/STBY switch (RHS)
  Device input has a unique bit for each switch or knob,
     the selector switch is always returned
  Any change will trigger one or two outputs -
     for selectors, one output
     for switch or knob, two outputs with the bit set and then reset
     The bottom 8 bits indicate the rotary knob and direction,
     ms 4 bits for the bottom rotaries, ls 4 bits for the top
  The Nasal code in FGFS expects the bit values as the action i.e. 1,2,4,8
  Reading the device produces three u8 values.
  Writing the device requires a byte of zero, followed by the display values.
  For Windows compatibility, an extra two bytes (value unimportant) are required,
  without this affecting the Linux code.
  The display u8s encode the numeric display - see 'fn convert_for_display'.
*/

pub struct Device {
    // ctxt: HidApi,
    device: HidDevice,
    // for device reads and writes
    dampf: bool,
    // to make rotary controls less sensitive
    input_current: u32,
    // data from device
    input_old: u32,     // previous data
}

impl Device {
    /// Create an instance of the Saitek Radio device.
    /// The (first) device is located by vendor and device ID.
    /// The device is initialised, providing a value for the Selection switch
    /// on the LHS of the panel, and the device is now set for non-blocking reads
    pub fn new() -> Device {
        let ctxt = HidApi::new().unwrap();
        let mut r = Device {
            // ctxt: HidApi::new().unwrap(),
            device: ctxt.open(VENDOR_ID, RADIO_ID).unwrap_or_else(|_err| {
                println!("Saitek Radio not found");
                process::exit(1);
            }),
            dampf: false,     // for damping the rotary switches
            input_current: 0, // adjusted during initialise_device
            input_old: 0,
        };
        // set up display & read selections, device is currently a blocking read
        r.input_current = initialise_device(&r.device) & SELECTIONS;
        // now make the device non-blocking
        r.device.set_blocking_mode(false).unwrap();
        r
    }

    /// Non-blocking read of the device into the 'input_current' field in the Device
    /// struct. If there is no data, does not disturb the 'input_current' field.
    /// Three data bytes are provided by the radio and are packed into a u32 such that
    /// the bit positions and other masks in 'radio_constants.rs' coincide.
    pub fn read(&mut self) {
        // Non-blocking read the radio panel switches and selectors
        // return 0 if no data, else pack 3 bytes into ls part of u32
        let mut buf = [0u8, 0, 0];
        let read_length = match self.device.read(&mut buf) {
            Ok(l) => l,  // good read, should be 3
            Err(_) => 0, // probably non-blocking - no data
        };
        // device sends 3 bytes, 0 on error
        if read_length == 3 {
            self.input_current = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        }
    }

    /// Write the contents of the data buffer to the device. This has been encoded for display.
    pub fn write(&self, data: &mut [u8; RIGHT_SIZE]) {
        // Send a feature report containing 0 followed by 20 u8's, 5 per display
        // and the extra 2 unimportant bytes
        data[0] = 0;
        self.device.send_feature_report(data).unwrap();
    }

    /// The display buffer is formed suitably for the selected radios, calling on the Simulator module
    /// for the latest values from FGFS. The 'write' fn is called to output the data.
    pub fn update_display(&self, sim: &Simulator) {
        // for top and bottom
        //    find the &str in 'value' relating to the radio on display
        //    convert this &str into the data buffer using character mapping
        // Output the completed display to the radio panel
        let mut buffer = [SPACE; RIGHT_SIZE]; // space filled for conversion routine!
        let mut offset: usize = 1;
        for row_mask in [TOP_SELECTIONS, BOTTOM_SELECTIONS].iter() {
            // Radio number is 0..7 for COM 1. COM 2, NAV 1, NAV 2, ADF, DME, XPDR
            let radio = find_radio(self.input_current & row_mask);
            let (act, stby) = sim.get_radio_data(radio);
            convert_to_display(&act, &mut buffer[offset..5 + offset]);
            convert_to_display(&stby, &mut buffer[offset + 5..10 + offset]);
            offset += 10;
        }
        self.write(&mut buffer);
    }

    /// Produces a tuple formatting a command for the Simulator to send to FGFS.
    /// If the action is a rotary knob twist, this is damped so only alternate clicks are sent,
    /// as the switches are quite sensitive. If there is nothing
    /// to send, the first element in the tuple is Option None, otherwise the tuple contains
    /// (Some(radio number), action code). The radio number is 0..7, the action code is 0,1,2,4,8
    pub fn make_command(&mut self) -> (Option<usize>, u8) {
        let c1 = [TOP_CONTROLS, BOTTOM_CONTROLS];
        let s1 = [TOP_SELECTIONS, BOTTOM_SELECTIONS];
        for (control, row) in c1.iter().zip(s1.iter()) {
            let switch = if row == &TOP_SELECTIONS { TOPSW } else { BOTSW };
            if (self.input_current & control) != 0 {
                // use this row
                let radio = find_radio(self.input_current & row);
                if self.input_current & switch != 0 {
                    // ACT/STBY
                    return (Some(radio), 0);
                } else {
                    // Rotary knobs
                    if self.dampf {
                        // every other click
                        let action: u8 = if row == &TOP_SELECTIONS {
                            (self.input_current & 0x0f) as u8
                        } else {
                            ((self.input_current & 0xf0) >> 4) as u8
                        };
                        self.dampf = false; // start damping for next action
                        return (Some(radio), action);
                    }
                    self.dampf = !self.dampf; // toggle damping
                }
            }
        }
        (None, 0)
    }

    /// Saves the current input as the old input
    pub fn preserve_current_input(&mut self) {
        self.input_old = self.input_current;
    }

    /// Compares the current input Control bits with the old input.
    /// Returns true if they differ.
    pub fn has_control_input_updated(&self) -> bool {
        (self.input_current & CONTROLS) != (self.input_old & CONTROLS)
    }

    /// Compares the selected radio switches for the current vs the old input.
    /// Return true if they differ.
    pub fn has_selection_input_updated(&self) -> bool {
        (self.input_current & SELECTIONS) != (self.input_old & SELECTIONS)
    }
}

/// Sends an initial string of digits to the display, waiting for a key to be pressed.
/// When key is received, blanks alternate characters in the display, and returns the
/// value read from the device, which gives the current display selections.
fn initialise_device(device: &HidDevice) -> u32 {
    // Do initialisation write
    let mut buf = [0u8; RIGHT_SIZE];
    for i in 1..10 {
        buf[i] = i as u8;
        buf[20 - i] = i as u8;
    }
    buf[0] = 0;
    device.send_feature_report(&buf).unwrap();
    println!("Operate ACT/STB key on the Saitek Radio");
    device.read(&mut buf).unwrap_or_else(|_| {
        println!("Saitek Radio read error");
        process::exit(3);
    });
    let reply = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32);
    println!("Saitek Radio ready");
    for i in 1..11 {
        buf[i * 2] = 0x0a;
    }
    buf[0] = 0;
    buf[1] = 1 as u8;
    device.send_feature_report(&buf).unwrap();
    reply
}

/// Returns an index 0..7 indicating the LH selector switch position.
/// Panics if not found
fn find_radio(instrument: u32) -> usize {
    // Finds the (simulator input) &str vec index for the selected instrument
    for i in 0..7 {
        if (instrument & RADIO_MAP[i]) != 0 {
            return i;
        }
    }
    panic!("Can't find instrument {:06x}", instrument);
}

/// Converts a string of up to 5 digits with an optional DP into a
/// right-justified buffer (initially space-filled) of 'magic' bytes for display by the radio device.
/// The buffer corresponds to one of the 4 display windows on the device.
/// The 'magic' bytes are those that cause the display to show the data, and are
/// converted from the b'0'..b'9' digits. A DP is produced by adjusting the code
/// for the digit before. Spaces are preserved as they occur. A '-' will leave the
/// output as spaces
fn convert_to_display(value: &String, display: &mut [u8]) {
    // Each of four displays consists of 5 u8 digit positions
    // encoded as 0..9 for '0'..'9', 0x0a for space, and with
    // added 0xd0 for trailing decimal point on a digit.
    // Code assumes display is already space filled, so can
    // start conversion at suitable position within the display.
    let dp = value.contains('.');
    let field = 5 + (if dp { 1 } else { 0 });
    let mut i = field - value.len();
    if i < field {
        // not empty
        for b in value.bytes() {
            if b == b'-' {
                return;
            }
            if b == b'.' {
                display[i - 1] += DECFLAG;
                continue;
            } else if b == b' ' {
                display[i] = SPACE;
            } else {
                display[i] = b - NUMBASE;
            }
            i += 1;
        }
    }
}
