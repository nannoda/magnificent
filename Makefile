# Makefile for building and running the Magnificent project

# Variables
APPLICATION_ID = com.nannoda.Magnificent
PROJECT_NAME = magnificent
FLATPAK_APP_DIR = flatpak_app
BUILD_JSON = build-aux/$(APPLICATION_ID).Devel.json

# Targets
.PHONY: all create-project install-deps build run clean

all: install-deps build

create-project:
	python3 create-project.py

install-deps:
	flatpak install --user org.gnome.Sdk//47 \
		org.gnome.Platform//47 \
		org.freedesktop.Sdk.Extension.rust-stable//24.08 \
		org.freedesktop.Sdk.Extension.llvm18//24.08

build:
	flatpak-builder --user $(FLATPAK_APP_DIR) $(BUILD_JSON)

run:
	flatpak-builder --run $(FLATPAK_APP_DIR) $(BUILD_JSON) $(PROJECT_NAME)

clean:
	rm -rf $(FLATPAK_APP_DIR) contrast
