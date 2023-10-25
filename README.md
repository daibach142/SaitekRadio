# FGFS DRIVER FOR SAITEK RADIO PANEL
   
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

Download the latest release from https://github.com/daibach142/SaitekRadio/releases/, and extract the contents into a working directory
(which may be discarded after installation is complete).

Ensure `flightgear` is installed first!

### On Linux x86-64:


Go to the working directory and type:

		sudo make install

If the Radio Panel is plugged in, disconnect and reconnect it to change the access
permissions.
	
See **RUNNING** at the end of this document		


-----

### On Windows:

1. Copy `saitekradio.exe` and `startup.bat` to a convenient location, most likely Desktop.
1. Copy `saitekradio.xml` to `C:\Program Files\Flightgear 2020.3\data\Protocol`

1. You may need to create the folder `Nasal` below.

1. Copy `saitekradio.nas` to `<username>\Appdata\Roaming\flightgear.org\Nasal`

See **RUNNING** at the end of this document		


-----

### On other systems eg. Raspberry Pi

1. Change your working directory to `code/saitekradio`
 
1. Install Rust - see `https://www.rust-lang.org/tools/install`

1. Run make

	`make`
	
	This creates a new `saitekradio` and replaces the original `saitekradio` with the new one.
	
1. Change directory to the installation directory  

	`cd ../..`

1. Now follow the instructions given for x86-64 above.

#### Issues

If the compilation fails, with an error message about `libusb`, you should install the package `libusb-1.0-0-dev` (on Ubuntu)
using the package manager. 

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

If the handler can't find/access the radio device, unplug it and replug and try again.


----

Note: the `saitekradio` executable will accept parameters:

- `saitekradio[.exe] [output IP address [input IP address]]`

if you wish to run it on a remote machine.


IMPORTANT NOTE!!!
If you have used my previous driver (FGFS_Saitek_Radio) you MUST remove the 'Saitek.nas'
file from your local 'Nasal' directory.



