#FGFS DRIVER FOR SAITEK RADIO PANEL
   
This project provides an interface between Flightgear Flight Simulator and the
Saitek Radio Panel. The simulated aircraft's radio settings are shown in
real time on the RadioPanel's display, and the ACT/STBY switches and the rotary
control operation is handed to the Simulator for action. 

Executable images are provided for Linux (x86-64) and Windows. The code base is identical
for Linux and Windows and is written in Rust; instructions are provided for making executables for other architectures
such as Raspberry Pi.

If you would like to improve the code or automate the Windows installation, please issue a 'pull' request. 

This project is the successor to daibach142/FGFS_Saitek_Radio on GitHub.

For problems or issues either enter an issue on Github, or email stkrp@attwoods.org.uk.

---

## INSTALLATION

Download project from Github, and extract the contents into a working directory
(which may be discarded after installation is complete).

### On Linux x86-64:

Type:

		sudo make install

If the Radio Panel is plugged in, disconnect and reconnect it to change the access
permissions.
	
See **RUNNING** at the end of this document		


-----

### On Windows:

1. Copy `saitekradio.exe` and `startup.bat` to a convenient location, most likely Desktop.
1. Copy `saitekradio.xm`l to `C:/Program Files/Flightgear 2020.3/data/Protocol`

1. You may need to create the directory(folder) `Nasal` below.

1. Copy `saitekradio.nas` to `<username>/Appdata/Roaming/flightgear.org/Nasal`

See **RUNNING** at the end of this document		


-----

### On other systems eg. Raspberry Pi

1. Change your working directory to `code/saitekradio`
 
1. Install Rust - see `https://www.rust-lang.org/tools/install`

1. Compile the code  

	`cargo build --release`
1. On Linux, reduce the size of the executable with  

	`strip target/release/saitekradio`
1. Copy the executable over the released `saitekradio[.exe]` from `target/release/saitekradio`

	`cp target/release/saitekradio ../..`
1. There is a `Makefile` in `code/saitekradio`, so you may perform steps 3 to 5 above with 
   
	`make`	 
1. Change directory to the installation directory  

	`cd ../..`

1. Now follow the instructions given for x86-64 above.

----

## RUNNING

Plug the Saitek Radio into a USB port.

Start the handler by running `saitekradio` (Linux) or double click `startup.bat` on Windows.

The radio display will show ascending/descending digits on the Radio display.
Press either of the ACT/STBY keys, and the display will blank alternate digits.

In the FGFS Start Screen, in `Settings/Additional settings`, insert the following:

`--generic=socket,out,30,127.0.0.1,60001,udp,saitekradio`<br>
`--generic=socket,in,20,,60002,udp,saitekradio`

and start the simulator.

Shortly, the display will change to show the selected radio details.
When the aircraft has fully loaded, all the radio panel controls will be active.


If the program terminates unexpectedly, re-run under the command-line interface
to be able to read the (hopefully useful) error message. 


----

Note: the `saitekradio` executable will accept parameters:

- `saitekradio[.exe] [output IP address [input IP address]]`

if you wish to run it on a remote machine.


IMPORTANT NOTE!!!
If you have used my previous driver (FGFS_Saitek_Radio) you MUST remove the 'Saitek.nas'
file from your local 'Nasal' directory.



