A rust crate to interact with the mainline Linux charlcd.c driver.

[![Latest version](https://img.shields.io/crates/v/charlcd.svg)](https://crates.io/crates/charlcd)
[![Documentation](https://docs.rs/charlcd/badge.svg)](https://docs.rs/charlcd)
[![License](https://img.shields.io/crates/l/charlcd.svg)](https://crates.io/crates/charlcd)

Or, said boldly: _a crate to **correctly** interact with HD44780 LCD
screens on embedded Linux devices_.

![Photo of a test on HD44780 20x4
screen](https://crates.microjoe.org/charlcd/media/docs/test.jpg)

A lot of developers are relying on userspace libraries to interact with the
very popular HD44780 LCD screens, but these implementations are lacking
proper abstractions and usually all reimplement the same communication
protocol to interact with the screen over and over.

This approach seems just wrong if you consider what the Linux kernel can
provide in comparison. Indeed, it is the role of device drivers to properly
abstract the access to the hardware from userspace perspective.  For this,
the Linux kernel uses an absraction mechanism to declare the hardware:
the *device-tree*.

The *device-tree* nodes for a screen behind an I2C GPIO expander would look
like the following:

```dts
&i2c0 {
    status = "okay";

    pcf8574a: i2c0@3f {
        compatible = "nxp,pcf8574a";
        reg = <0x3f>;
        gpio-controller;
        #gpio-cells = <2>;
    };
};

auxdisplay: auxdisplay {
    compatible = "hit,hd44780";

    data-gpios = <&pcf8574a 4 GPIO_ACTIVE_HIGH>,
                 <&pcf8574a 5 GPIO_ACTIVE_HIGH>,
                 <&pcf8574a 6 GPIO_ACTIVE_HIGH>,
                 <&pcf8574a 7 GPIO_ACTIVE_HIGH>;
    rs-gpios = <&pcf8574a 0 GPIO_ACTIVE_HIGH>;
    rw-gpios = <&pcf8574a 1 GPIO_ACTIVE_HIGH>;
    enable-gpios = <&pcf8574a 2 GPIO_ACTIVE_HIGH>;
    backlight-gpios = <&pcf8574a 3 GPIO_ACTIVE_LOW>;

    display-height-chars = <2>;
    display-width-chars = <16>;
};
```

Once the device-tree is correctly configured, the `charlcd` driver will
create a `/dev/lcd` character device.

The role of this library is to provide an astraction over this character
device entry, while letting the kernel driver implement the communication
with the screen — instead of going from scratch and using ioctl over
`/dev/i2c-*` like many other libraries do.

For more information about this and a demo on *real hardware*, see [this blog
article](https://blog.microjoe.org/2019/hd44780-lcd-i2c-screen-using-linux-mainline-charlcd-driver.html).
