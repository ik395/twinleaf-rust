# Twinleaf I/O Tools in Rust

This repository contains a set of tools that are useful for working with Twinleaf quantum sensors and accessories. 

### tio-proxy

The primary tool is the proxy, which makes the device available via ethernet:

		tio-proxy --auto

When there are more than one serial port available, it is necessary to specify the port

		[linux]> tio-proxy -r /dev/ttyACM0
		[macOS]> tio-proxy -r /dev/cu.usbserialXXXXXX
		[wsl1] > tio-proxy -r COM3

With the proxy running, a set of tools can be used on the data stream. 

### tio-tool

Logging data:
		
		tio-tool log

Issuing commands:
		
		tio-tool rpc dev.name

There are a variety of additional useful functions. 
To see all tool options run:

		tio-tool --help

### tio-monitor
Displays a live stream of incoming data.

Running the tool:

		tio-monitor usb 


## Installation

With rust language tools, install the tools using:

		cargo install twinleaf-tools


## Development

On linux, there is a dependency on libudev; to install it use:

		sudo apt install libudev-dev  # debian linux

## Cross compilation 

The tools can be compiled for other platforms by first adding those platform targets:

		rustup target add x86_64-pc-windows-gnu
		rustup toolchain install stable-x86_64-pc-windows-gnu

And then building for the new target:

		cargo build --target x86_64-pc-windows-gnu
