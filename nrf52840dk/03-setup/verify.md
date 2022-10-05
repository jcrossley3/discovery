# Verify the installation

Let's verify that all the tools were installed correctly.

## Linux only

### Verify permissions

Connect the nRF52840-DK to your computer using a USB cable.

The nRF52840-DK should now appear as a USB device (file) in `/dev/bus/usb`. Let's find out how it got
enumerated:

``` console
$ lsusb | grep -i "segger j-link"
Bus 003 Device 020: ID 1366:1015 SEGGER J-Link
$ # ^^^        ^^^
```

In my case, the nRF52840-DK got connected to the bus #3 and got enumerated as the device #20. This means the
file `/dev/bus/usb/003/020` *is* the nRF52840-DK. Let's check its permissions:

``` console
$ ls -l /dev/bus/usb/003/020
crw-rw-rw-+ 1 root root 189, 275 Oct  5 10:37 /dev/bus/usb/003/020
```

The permissions should be `crw-rw-rw-`. If it's not ... then check your [udev
rules] and try re-loading them with:

[udev rules]: linux.md#udev-rules

``` console
$ sudo udevadm control --reload-rules
```

# All

## Verifying cargo-embed
First, connect the nRF52840-DK to your Computer using a USB cable.

At least one LED on the nRF52840-DK should light up.

Next run one of these commands:

```
$ # make sure you are in the 03-setup/ dir
$ rustup target add thumbv7em-none-eabihf
$ cargo embed
```

If everything works correctly cargo-embed should first compile the small example program
in this directory, then flash it and finally open a nice text based user interface that
prints Hello World.

(If it does not, check out [general troubleshooting] instructions.)

[general troubleshooting]: ../appendix/1-general-troubleshooting/index.html

This output is coming from the small Rust program you just flashed on to your nRF52840-DK.
Everything is working properly and you can continue with the next chapters!
