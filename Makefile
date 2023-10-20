#
# Install and remove package contents
#
#
#
#
PRODUCT = saitekradio

SHELL := /bin/bash # Use bash syntax
# This 'make' must be run with 'sudo'
LOCAL_HOME = /home/$(SUDO_USER)
X11_CONF = /usr/share/X11/xorg.conf.d
UDEV_RULES = /etc/udev/rules.d

LOCAL_FGFS = $(LOCAL_HOME)/.fgfs

GAMES_PATH = /usr/games
SYSTEM_FGFS = /usr/share/games/flightgear

PHONY: install uninstall 

install:
	$(info )
	$(info *********** Local FGFS location is $(LOCAL_FGFS) ************* )
	$(info )
	cp $(PRODUCT) $(GAMES_PATH)
	cp *.xml $(SYSTEM_FGFS)/Protocol
	mkdir -pv $(LOCAL_FGFS)/Nasal
	cp saitekradio.nas $(LOCAL_FGFS)/Nasal
	cp 55-saitekpanels.conf $(X11_CONF)
	cp 55-saitek.rules $(UDEV_RULES)
	udevadm control --reload

uninstall:
	$(info )
	$(info *********** Local FGFS location is $(LOCAL_FGFS) ************* )
	$(info )
	-pkill -9 saitekradio
	-rm $(GAMES_PATH)/$(PRODUCT)
	-rm $(SYSTEM_FGFS)/Protocol/saitekradio.xml
	-rm $(LOCAL_FGFS)/Nasal/saitekradio.nas
	-rm $(X11_CONF)/55-saitekpanels.conf
	-rm $(UDEV_RULES)/55-saitek.rules
	udevadm control --reload






