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

/// Definitions for the Radio selections (left hand side switches)
/// as read from the radio device
pub const TOPCOM1: u32 = 0x010000;
pub const TOPCOM2: u32 = 0x020000;
pub const TOPNAV1: u32 = 0x040000;
pub const TOPNAV2: u32 = 0x080000;
pub const TOPADF: u32 = 0x100000;
pub const TOPDME: u32 = 0x200000;
pub const TOPXPDR: u32 = 0x400000;
pub const BOTCOM1: u32 = 0x800000;
pub const BOTCOM2: u32 = 0x000100;
pub const BOTNAV1: u32 = 0x000200;
pub const BOTNAV2: u32 = 0x000400;
pub const BOTADF: u32 = 0x000800;
pub const BOTDME: u32 = 0x001000;
pub const BOTXPDR: u32 = 0x002000;

/// For classification
pub const TOP_SELECTIONS: u32 = TOPCOM1 | TOPCOM2 | TOPNAV1 | TOPNAV2 | TOPADF | TOPDME | TOPXPDR;
pub const BOTTOM_SELECTIONS: u32 =
    BOTCOM1 | BOTCOM2 | BOTNAV1 | BOTNAV2 | BOTADF | BOTDME | BOTXPDR;
pub const SELECTIONS: u32 = TOP_SELECTIONS | BOTTOM_SELECTIONS;

// Definitions for the rotary switches and ACT/STBY switches
// as read from the radio device
pub const TOPSW: u32 = 0x004000;
pub const BOTSW: u32 = 0x008000;
pub const TOPINCD: u32 = 0x000001;
pub const TOPDECD: u32 = 0x000002;
pub const TOPINC: u32 = 0x000004;
pub const TOPDEC: u32 = 0x000008;
pub const BOTINCD: u32 = 0x000010;
pub const BOTDECD: u32 = 0x000020;
pub const BOTINC: u32 = 0x000040;
pub const BOTDEC: u32 = 0x000080;

// For classification
pub const TOP_ROTARY: u32 = TOPINCD | TOPDECD | TOPINC | TOPDEC;
pub const BOTTOM_ROTARY: u32 = BOTINCD | BOTDECD | BOTINC | BOTDEC;

pub const TOP_CONTROLS: u32 = TOP_ROTARY | TOPSW;
pub const BOTTOM_CONTROLS: u32 = BOTTOM_ROTARY | BOTSW;
pub const CONTROLS: u32 = TOP_CONTROLS | BOTTOM_CONTROLS;

// This maps the radio to the index into the simulator incoming values
pub const RADIO_MAP: [u32; 7] = [
    TOPCOM1 | BOTCOM1,
    TOPCOM2 | BOTCOM2,
    TOPNAV1 | BOTNAV1,
    TOPNAV2 | BOTNAV2,
    TOPADF | BOTADF,
    TOPDME | BOTDME,
    TOPXPDR | BOTXPDR,
];

// For the Saitek Radio Panel, following funny constants

// Add into byte to set decimal point after on Panel display
pub const DECFLAG: u8 = 0xD0;
// Produce a blank position byte on Panel display
pub const SPACE: u8 = 0x0A;
// Subtract from numeric digit to get correct digit on Panel display
pub const NUMBASE: u8 = b'0';
