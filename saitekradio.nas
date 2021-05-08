####################################################
#     FUNCTIONS in Saitek Radio Panel
####################################################

####################################################
# increment the transponder digit
# range per digit is 0..7, no wrap
# digit position saved in xpdr_cpos
####################################################
var increment_xpdr = func (action) {
	var index = getprop("saitek_radio_panel/xpdr_cp");
	var digitv = getprop("instrumentation/transponder/inputs/digit["~math.floor(index)~"]");
	if (action == 1 or action == 4) {
		digitv = digitv + 1;
	} else {
		digitv = digitv - 1;
	}
	if (digitv < 0) digitv = 7;
	if (digitv > 7) digitv = 0;
	setprop("instrumentation/transponder/inputs/digit["~index~"]", digitv);	
}

####################################################
# increment the kHz radio (adf)
####################################################
var increment_khz = func (device, increment, base, top) {
	var newval = getprop("instrumentation/"~device~"/frequencies/standby-khz");
	newval = newval + increment;
	if (newval < base) newval = top
	else if (newval > top) newval = base;
	setprop("instrumentation/"~device~"/frequencies/standby-khz", newval);
}
	
#################################################
# increment the mHz radio (comm or nav)
####################################################
var increment_mhz = func (device, increment, base, top) {
	var newval = getprop("instrumentation/"~device~"/frequencies/standby-mhz");
	newval = newval + increment;
	if (newval < base) newval = top;
	else if (newval >= top) newval = base;
	setprop("instrumentation/"~device~"/frequencies/standby-mhz", newval);
}

####################################################
# Sets up arguments to call device incrementors
# variable limts and increments depnding on device
# device is (string) comm, comm[1], nav, nav[1], adf,
# transponder.
# action is 1 - small increment, 2 - small decrement
#           4 - big increment,   8 - big decrement
####################################################    
var do_increment = func (device, action) {
	if (device == "transponder") {
		increment_xpdr(action);
		return;
	}
	var increment = 1; 
	var range_base = 108; 
	var range_top = 117;
	if (streq(substr(device,0,1),"a")) {
		# adf
		increment = (action == 1 or action == 2) ? 1 : 25;
		if (action == 2 or action == 8) increment = -increment;
		increment_khz (device, increment, 200, 1700); 
	} else {
		# com or nav
		# com or nav different limits
		if ( streq (substr (device,0,1), "c")) {
			# com, com1
			increment = (action == 1 or action == 2) ? 0.01 : 1.00;
			range_base = 118.00; range_top = 137.000;
		} else {
			# nav, nav1
			range_base = 108.00; range_top = 118.00;
			increment = (action == 1 or action == 2) ? 0.05 : 1.00;
		}
		if (action == 2 or action == 8) increment = -increment;
		increment_mhz(device, increment, range_base, range_top);
	}	
}

####################################################
# Perform ACT/STBY switching for all 
# and call handlers for other actions
####################################################
var do_action = func {
	var device = getprop("saitek_radio_panel/device");
	var action = getprop("saitek_radio_panel/action");
	
	# There are no actions for dme - it's just a display
	if (device != "dme") {
	### vector out to handlers for different types
	### then the actions can be processed from there
		if (action == "0") {
		# swap   act/standby
			var propFreq = "instrumentation/"~device~"/frequencies/";
			if (streq(substr(device,0,1),"a")) {
				# adf
				var standby = getprop(propFreq~"standby-khz");
				var selected = getprop(propFreq~"selected-khz");
				setprop (propFreq~"standby-khz", selected);
				setprop (propFreq~"selected-khz", standby);
			}
			else if (device == "transponder") {
				# move to next digit position
				var cpos = getprop("saitek_radio_panel/xpdr_cp");
				cpos = cpos + 1;
				if (cpos > 3) cpos = 0;
				setprop("saitek_radio_panel/xpdr_cp", cpos);
			}
			else {
				# comm, comm[1], nav, nav[1] - mHz radios
				var standby = getprop(propFreq~"standby-mhz");
				var selected = getprop(propFreq~"selected-mhz");
				setprop (propFreq~"standby-mhz", selected);
				setprop (propFreq~"selected-mhz", standby);
			}
		} else {
			# not swap action - must be change value
			do_increment(device, action);
		}	
	} # not dme
}

setlistener ("/saitek_radio_panel/action", do_action);

# holder for digit selection on the transponder
setprop ("/saitek_radio_panel/xpdr_cp", 0);

## The input has saved the device and the action in the saitek_radio_panel

