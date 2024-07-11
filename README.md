# macro-key-config

Part of this project:
https://www.printables.com/model/938192-18-key-macro-keyboard


This project aims to provide a macro keyboard firmware and configuration application, besides all community solutions available, I wanted to introduce myself on Rust Language, Slint GUI, Arduino and 3D Printing.

**The arduino source code uses the following libraries:**
* Default Arduino Keyboard
* Keypad Library for Arduino [Arduino Playground](http://playground.arduino.cc/Code/Keypad)
* Modified HID-Project, only for RawHID  [HID-Project](https://github.com/NicoHood/HID)

You can use Arduino IDE for building and uploading, just set the libraries in place and hit the buttons.




**The Rust Config Application uses the following libraries:**
* hidapi = "2.6.1" [Hidapi](https://github.com/libusb/hidapi)
* slint = "1.6" [Slint](https://slint.dev/)

Type this and see what happens

```sh
cargo run
```


There are some issues on windows, haven't tested on mac, runs ok on linux.

Suggestions are welcome...