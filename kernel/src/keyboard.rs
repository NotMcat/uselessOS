use spin::Mutex;

pub struct Keyboard {
    maiusc: bool,
    shift: bool,
    shift2: bool,
    ctrl: bool,
    numpad: bool,
    alt: bool,
    alt2: bool,
}

pub static mut KEYBOARD: Keyboard = Keyboard {
    maiusc: false,
    shift: false,
    shift2: false,
    ctrl: false,
    numpad: true,
    alt: false,
    alt2: false,
};

pub fn keyboard_italian(char: u8) {
    let key = match char {
        0x01 => Some("Esc"),
        0x02 => two_ways("!","1"),
        0x03 => two_ways("\"","2"),
        0x04 => two_ways("£","3"),
        0x05 => two_ways("$","4"),
        0x06 => two_ways("%","5"),
        0x07 => two_ways("&","6"),
        0x08 => two_ways("/","7"),
        0x09 => two_ways("(","8"),
        0x0A => two_ways(")","9"),
        0x0B => two_ways("=","0"),
        0x0C => two_ways("?","'"),
        0x0D => two_ways("^", "i'"),
        0x0E => Some(""),
        0x0F => {
            unsafe{ KEYBOARD.alt = true };
            Some("")
        },
        0x10 => lett_alt("Q", "q"),
        0x11 => lett_alt("W", "w"),
        0x12 => lett_alt("E", "e"),
        0x13 => lett_alt("R", "r"),
        0x14 => lett_alt("T", "t"),
        0x15 => lett_alt("Y", "y"),
        0x16 => lett_alt("U", "u"),
        0x17 => lett_alt("I", "i"),
        0x18 => lett_alt("O", "o"),
        0x19 => lett_alt("P", "p"),
        0x1A => four_ways("e'", "[", "e'" , "{"),
        0x1B => four_ways("+", "]", "*", "}"),
        0x1C => Some("\n"),
        0x3A => {
            let v = unsafe{ KEYBOARD.maiusc };
            unsafe { KEYBOARD.maiusc = !v };
            Some("")
        },
        0x2A => {
            unsafe { KEYBOARD.shift = true };
            Some("")
        },
        0x1E => lett_alt("A", "a"),
        0x1F => lett_alt("S", "s"),
        0x20 => lett_alt("D", "d"),
        0x21 => lett_alt("F", "f"),
        0x22 => lett_alt("G", "g"),
        0x23 => lett_alt("H", "h"),
        0x24 => lett_alt("J", "j"),
        0x25 => lett_alt("K", "k"),
        0x26 => lett_alt("L", "l"),
        0x27 => three_ways("o'", "@", "ç"),
        0x28 => three_ways("a'","#","°"),
        0x29 => two_ways("|", "\\"),
        0x2B => two_ways("§","ù"),
        0x56 => two_ways(">","<"),
        0x2C => lett_alt("Z", "z"),
        0x2D => lett_alt("X", "x"),
        0x2E => lett_alt("C", "c"),
        0x2F => lett_alt("V", "v"),
        0x30 => lett_alt("B", "b"),
        0x31 => lett_alt("N", "n"),
        0x32 => lett_alt("M", "m"),
        0x33 => two_ways(";", ","),
        0x34 => two_ways(":", "."),
        0x35 => two_ways("_", "-"),
        0x36 => {
            unsafe { KEYBOARD.shift2 = true };
            Some("")
        },
        0x1D => {
            unsafe { KEYBOARD.ctrl = true };
            Some("")
        },
        0x38 => {
            unsafe { KEYBOARD.alt = true };
            Some("")
        },
        0x39 => Some(" "),
        0x3B => Some("F1"),
        0x3C => Some("F2"),
        0x3D => Some("F3"),
        0x3E => Some("F4"),
        0x3F => Some("F5"),
        0x40 => Some("F6"),
        0x41 => Some("F7"),
        0x42 => Some("F8"),
        0x43 => Some("F9"),
        0x44 => Some("F10"),
        0x57 => Some("F11"),
        0x58 => Some("F12"),
        0x52 => num_alt("0"),
        0x53 => Some("."),
        0x4F => num_alt("1"),
        0x50 => num_alt("2"),
        0x51 => num_alt("3"),
        0x4B => num_alt("4"),
        0x4C => num_alt("5"),
        0x4D => num_alt("6"),
        0x47 => num_alt("7"),
        0x48 => num_alt("8"),
        0x49 => num_alt("9"),
        0x4E => Some("+"),
        0x4A => Some("-"),
        0x37 => Some("*"),
        0x45 => {
            let numpad = unsafe{ KEYBOARD.numpad };
            unsafe{ KEYBOARD.numpad = !numpad };
            Some("")
        },
        0xAA => {
            unsafe{ KEYBOARD.shift = false };
            Some("")
        },
        0xB8 => {
            unsafe{ KEYBOARD.alt = false };
            Some("")
        },

        0xB6 => {
            unsafe{ KEYBOARD.shift2 = false };
            Some("")
        },


        _ => None,
    };

    if let Some(key) = key {
        if key != "" {
            unsafe{ crate::serial::TERMINAL.write_kb(key) };
        }
    }

}

fn num_alt(num: &str) -> Option<&str> {
    if unsafe{ KEYBOARD.numpad } {
        Some(num)
    } else {
        Some("")
    }
}

fn lett_alt<'a>(ch: &'a str, ch2: &'a str) -> Option<&'a str> {

    let sh1 = unsafe{ KEYBOARD.shift };
    let sh2 = unsafe{ KEYBOARD.shift2 };
    let maiusc = unsafe{ KEYBOARD.maiusc };

    if sh1 && maiusc || sh2 && maiusc {
        Some(ch2)
    } else if sh1 || maiusc || sh2 {
        Some(ch)
    } else {
        Some(ch2)
    }
}

fn two_ways<'a>(ch: &'a str, ch2: &'a str) -> Option<&'a str> {
    let sh1 = unsafe{ KEYBOARD.shift };
    let sh2 = unsafe{ KEYBOARD.shift2 };

    if sh1 || sh2 {
        Some(ch)
    } else {
        Some(ch2)
    }
}

fn four_ways<'a>(ch: &'a str, ch2: &'a str, ch3: &'a str, ch4: &'a str) -> Option<&'a str> {
    let sh = unsafe{ KEYBOARD.shift };
    let sh2 = unsafe{ KEYBOARD.shift2 };
    let alt = unsafe{ KEYBOARD.alt };
    let alt2 = unsafe{ KEYBOARD.alt2 };

    if sh && alt || sh2 && alt2 || sh && alt2 || sh2 && alt{
        Some(ch4)
    } else if sh || sh2{
        Some(ch3)
    } else if alt || alt2 {
        Some(ch2)
    } else {
        Some(ch)
    }
}

fn three_ways<'a>(ch: &'a str, ch2: &'a str, ch3: &'a str) -> Option<&'a str> {
    let sh = unsafe{ KEYBOARD.shift };
    let sh2 = unsafe{ KEYBOARD.shift2 };
    let alt = unsafe{ KEYBOARD.alt };
    let alt2 = unsafe{ KEYBOARD.alt2 };

    if sh || sh2{
        Some(ch3)
    } else if alt || alt2 {
        Some(ch2)
    } else {
        Some(ch)
    }
}