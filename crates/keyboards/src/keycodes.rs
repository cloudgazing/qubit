#![expect(clippy::doc_markdown)]

//! USB HID Keyboard modifier masks and scan codes as per USB spec.

use core::num::NonZeroU8;

// -- Modifier masks --
//
// Used for the first byte in the HID report.

pub const KM_LCTRL: NonZeroU8 = NonZeroU8::new(0x01).unwrap();
pub const KM_LSHIFT: NonZeroU8 = NonZeroU8::new(0x02).unwrap();
pub const KM_LALT: NonZeroU8 = NonZeroU8::new(0x04).unwrap();
pub const KM_LMETA: NonZeroU8 = NonZeroU8::new(0x08).unwrap();
pub const KM_RCTRL: NonZeroU8 = NonZeroU8::new(0x10).unwrap();
pub const KM_RSHIFT: NonZeroU8 = NonZeroU8::new(0x20).unwrap();
pub const KM_RALT: NonZeroU8 = NonZeroU8::new(0x40).unwrap();
pub const KM_RMETA: NonZeroU8 = NonZeroU8::new(0x80).unwrap();

/// The Second byte in the report, which is reserved.
pub const RESERVED: u8 = 0x00;

// -- Scan codes. --
//
// Last N slots in the HID report (usually 6).
// 0x00 if no key pressed.
//
// If more than N keys are pressed, the HID reports
// KC_ERR_OVF in all slots to indicate this condition.

/// No key pressed
pub const KC_NONE: u8 = 0x00;
/// Keyboard Error Roll Over
///
/// Used for all slots if too many keys are pressed ("Phantom key")
pub const KC_ERR_OVF: NonZeroU8 = NonZeroU8::new(0x01).unwrap();

// 0x02 //  Keyboard POST Fail
// 0x03 //  Keyboard Error Undefined

/// Keyboard a and A
pub const KC_A: NonZeroU8 = NonZeroU8::new(0x04).unwrap();
/// Keyboard b and B
pub const KC_B: NonZeroU8 = NonZeroU8::new(0x05).unwrap();
/// Keyboard c and C
pub const KC_C: NonZeroU8 = NonZeroU8::new(0x06).unwrap();
/// Keyboard d and D
pub const KC_D: NonZeroU8 = NonZeroU8::new(0x07).unwrap();
/// Keyboard e and E
pub const KC_E: NonZeroU8 = NonZeroU8::new(0x08).unwrap();
/// Keyboard f and F
pub const KC_F: NonZeroU8 = NonZeroU8::new(0x09).unwrap();
/// Keyboard g and G
pub const KC_G: NonZeroU8 = NonZeroU8::new(0x0A).unwrap();
/// Keyboard h and H
pub const KC_H: NonZeroU8 = NonZeroU8::new(0x0B).unwrap();
/// Keyboard i and I
pub const KC_I: NonZeroU8 = NonZeroU8::new(0x0C).unwrap();
/// Keyboard j and J
pub const KC_J: NonZeroU8 = NonZeroU8::new(0x0D).unwrap();
/// Keyboard k and K
pub const KC_K: NonZeroU8 = NonZeroU8::new(0x0E).unwrap();
/// Keyboard l and L
pub const KC_L: NonZeroU8 = NonZeroU8::new(0x0F).unwrap();
/// Keyboard m and M
pub const KC_M: NonZeroU8 = NonZeroU8::new(0x10).unwrap();
/// Keyboard n and N
pub const KC_N: NonZeroU8 = NonZeroU8::new(0x11).unwrap();
/// Keyboard o and O
pub const KC_O: NonZeroU8 = NonZeroU8::new(0x12).unwrap();
/// Keyboard p and P
pub const KC_P: NonZeroU8 = NonZeroU8::new(0x13).unwrap();
/// Keyboard q and Q
pub const KC_Q: NonZeroU8 = NonZeroU8::new(0x14).unwrap();
/// Keyboard r and R
pub const KC_R: NonZeroU8 = NonZeroU8::new(0x15).unwrap();
/// Keyboard s and S
pub const KC_S: NonZeroU8 = NonZeroU8::new(0x16).unwrap();
/// Keyboard t and T
pub const KC_T: NonZeroU8 = NonZeroU8::new(0x17).unwrap();
/// Keyboard u and U
pub const KC_U: NonZeroU8 = NonZeroU8::new(0x18).unwrap();
/// Keyboard v and V
pub const KC_V: NonZeroU8 = NonZeroU8::new(0x19).unwrap();
/// Keyboard w and W
pub const KC_W: NonZeroU8 = NonZeroU8::new(0x1A).unwrap();
/// Keyboard x and X
pub const KC_X: NonZeroU8 = NonZeroU8::new(0x1B).unwrap();
/// Keyboard y and Y
pub const KC_Y: NonZeroU8 = NonZeroU8::new(0x1C).unwrap();
/// Keyboard z and Z
pub const KC_Z: NonZeroU8 = NonZeroU8::new(0x1D).unwrap();

/// Keyboard 1 and !
pub const KC_1: NonZeroU8 = NonZeroU8::new(0x1E).unwrap();
/// Keyboard 2 and @
pub const KC_2: NonZeroU8 = NonZeroU8::new(0x1F).unwrap();
/// Keyboard 3 and #
pub const KC_3: NonZeroU8 = NonZeroU8::new(0x20).unwrap();
/// Keyboard 4 and $
pub const KC_4: NonZeroU8 = NonZeroU8::new(0x21).unwrap();
/// Keyboard 5 and %
pub const KC_5: NonZeroU8 = NonZeroU8::new(0x22).unwrap();
/// Keyboard 6 and ^
pub const KC_6: NonZeroU8 = NonZeroU8::new(0x23).unwrap();
/// Keyboard 7 and &
pub const KC_7: NonZeroU8 = NonZeroU8::new(0x24).unwrap();
/// Keyboard 8 and *
pub const KC_8: NonZeroU8 = NonZeroU8::new(0x25).unwrap();
/// Keyboard 9 and (
pub const KC_9: NonZeroU8 = NonZeroU8::new(0x26).unwrap();
/// Keyboard 0 and )
pub const KC_0: NonZeroU8 = NonZeroU8::new(0x27).unwrap();

/// Keyboard Return (ENTER)
pub const KC_ENTER: NonZeroU8 = NonZeroU8::new(0x28).unwrap();
/// Keyboard ESCAPE
pub const KC_ESC: NonZeroU8 = NonZeroU8::new(0x29).unwrap();
/// Keyboard DELETE (Backspace)
pub const KC_BACKSPACE: NonZeroU8 = NonZeroU8::new(0x2A).unwrap();
/// Keyboard Tab
pub const KC_TAB: NonZeroU8 = NonZeroU8::new(0x2B).unwrap();
/// Keyboard Spacebar
pub const KC_SPACE: NonZeroU8 = NonZeroU8::new(0x2C).unwrap();
/// Keyboard - and _
pub const KC_MINUS: NonZeroU8 = NonZeroU8::new(0x2D).unwrap();
/// Keyboard = and +
pub const KC_EQUAL: NonZeroU8 = NonZeroU8::new(0x2E).unwrap();
/// Keyboard [ and {
pub const KC_LEFTBRACE: NonZeroU8 = NonZeroU8::new(0x2F).unwrap();
/// Keyboard ] and }
pub const KC_RIGHTBRACE: NonZeroU8 = NonZeroU8::new(0x30).unwrap();
/// Keyboard \ and |
pub const KC_BACKSLASH: NonZeroU8 = NonZeroU8::new(0x31).unwrap();
/// Keyboard Non-US # and ~
pub const KC_HASHTILDE: NonZeroU8 = NonZeroU8::new(0x32).unwrap();
/// Keyboard ; and :
pub const KC_SEMICOLON: NonZeroU8 = NonZeroU8::new(0x33).unwrap();
/// Keyboard ' and "
pub const KC_APOSTROPHE: NonZeroU8 = NonZeroU8::new(0x34).unwrap();
/// Keyboard ` and ~
pub const KC_GRAVE: NonZeroU8 = NonZeroU8::new(0x35).unwrap();
/// Keyboard , and <
pub const KC_COMMA: NonZeroU8 = NonZeroU8::new(0x36).unwrap();
/// Keyboard . and >
pub const KC_DOT: NonZeroU8 = NonZeroU8::new(0x37).unwrap();
/// Keyboard / and ?
pub const KC_SLASH: NonZeroU8 = NonZeroU8::new(0x38).unwrap();
/// Keyboard Caps Lock
pub const KC_CAPSLOCK: NonZeroU8 = NonZeroU8::new(0x39).unwrap();

/// Keyboard F1
pub const KC_F1: NonZeroU8 = NonZeroU8::new(0x3A).unwrap();
/// Keyboard F2
pub const KC_F2: NonZeroU8 = NonZeroU8::new(0x3B).unwrap();
/// Keyboard F3
pub const KC_F3: NonZeroU8 = NonZeroU8::new(0x3C).unwrap();
/// Keyboard F4
pub const KC_F4: NonZeroU8 = NonZeroU8::new(0x3D).unwrap();
/// Keyboard F5
pub const KC_F5: NonZeroU8 = NonZeroU8::new(0x3E).unwrap();
/// Keyboard F6
pub const KC_F6: NonZeroU8 = NonZeroU8::new(0x3F).unwrap();
/// Keyboard F7
pub const KC_F7: NonZeroU8 = NonZeroU8::new(0x40).unwrap();
/// Keyboard F8
pub const KC_F8: NonZeroU8 = NonZeroU8::new(0x41).unwrap();
/// Keyboard F9
pub const KC_F9: NonZeroU8 = NonZeroU8::new(0x42).unwrap();
/// Keyboard F10
pub const KC_F10: NonZeroU8 = NonZeroU8::new(0x43).unwrap();
/// Keyboard F11
pub const KC_F11: NonZeroU8 = NonZeroU8::new(0x44).unwrap();
/// Keyboard F12
pub const KC_F12: NonZeroU8 = NonZeroU8::new(0x45).unwrap();

/// Keyboard Print Screen
pub const KC_SYSRQ: NonZeroU8 = NonZeroU8::new(0x46).unwrap();
/// Keyboard Scroll Lock
pub const KC_SCROLLLOCK: NonZeroU8 = NonZeroU8::new(0x47).unwrap();
/// Keyboard Pause
pub const KC_PAUSE: NonZeroU8 = NonZeroU8::new(0x48).unwrap();
/// Keyboard Insert
pub const KC_INSERT: NonZeroU8 = NonZeroU8::new(0x49).unwrap();
/// Keyboard Home
pub const KC_HOME: NonZeroU8 = NonZeroU8::new(0x4A).unwrap();
/// Keyboard Page Up
pub const KC_PAGEUP: NonZeroU8 = NonZeroU8::new(0x4B).unwrap();
/// Keyboard Delete Forward
pub const KC_DELETE: NonZeroU8 = NonZeroU8::new(0x4C).unwrap();
/// Keyboard End
pub const KC_END: NonZeroU8 = NonZeroU8::new(0x4D).unwrap();
/// Keyboard Page Down
pub const KC_PAGEDOWN: NonZeroU8 = NonZeroU8::new(0x4E).unwrap();
/// Keyboard Right Arrow
pub const KC_RIGHT: NonZeroU8 = NonZeroU8::new(0x4F).unwrap();
/// Keyboard Left Arrow
pub const KC_LEFT: NonZeroU8 = NonZeroU8::new(0x50).unwrap();
/// Keyboard Down Arrow
pub const KC_DOWN: NonZeroU8 = NonZeroU8::new(0x51).unwrap();
/// Keyboard Up Arrow
pub const KC_UP: NonZeroU8 = NonZeroU8::new(0x52).unwrap();

/// Keyboard Num Lock and Clear
pub const KC_NUMLOCK: NonZeroU8 = NonZeroU8::new(0x53).unwrap();
/// Keypad /
pub const KC_KP_SLASH: NonZeroU8 = NonZeroU8::new(0x54).unwrap();
/// Keypad *
pub const KC_KP_ASTERISK: NonZeroU8 = NonZeroU8::new(0x55).unwrap();
/// Keypad -
pub const KC_KP_MINUS: NonZeroU8 = NonZeroU8::new(0x56).unwrap();
/// Keypad +
pub const KC_KP_PLUS: NonZeroU8 = NonZeroU8::new(0x57).unwrap();
/// Keypad ENTER
pub const KC_KP_ENTER: NonZeroU8 = NonZeroU8::new(0x58).unwrap();
/// Keypad 1 and End
pub const KC_KP_1: NonZeroU8 = NonZeroU8::new(0x59).unwrap();
/// Keypad 2 and Down Arrow
pub const KC_KP_2: NonZeroU8 = NonZeroU8::new(0x5A).unwrap();
/// Keypad 3 and PageDn
pub const KC_KP_3: NonZeroU8 = NonZeroU8::new(0x5B).unwrap();
/// Keypad 4 and Left Arrow
pub const KC_KP_4: NonZeroU8 = NonZeroU8::new(0x5C).unwrap();
/// Keypad 5
pub const KC_KP_5: NonZeroU8 = NonZeroU8::new(0x5D).unwrap();
/// Keypad 6 and Right Arrow
pub const KC_KP_6: NonZeroU8 = NonZeroU8::new(0x5E).unwrap();
/// Keypad 7 and Home
pub const KC_KP_7: NonZeroU8 = NonZeroU8::new(0x5F).unwrap();
/// Keypad 8 and Up Arrow
pub const KC_KP_8: NonZeroU8 = NonZeroU8::new(0x60).unwrap();
/// Keypad 9 and Page Up
pub const KC_KP_9: NonZeroU8 = NonZeroU8::new(0x61).unwrap();
/// Keypad 0 and Insert
pub const KC_KP_0: NonZeroU8 = NonZeroU8::new(0x62).unwrap();
/// Keypad . and Delete
pub const KC_KP_DOT: NonZeroU8 = NonZeroU8::new(0x63).unwrap();

/// Keyboard Non-US \ and |
pub const KC_102ND: NonZeroU8 = NonZeroU8::new(0x64).unwrap();
/// Keyboard Application
pub const KC_COMPOSE: NonZeroU8 = NonZeroU8::new(0x65).unwrap();
/// Keyboard Power
pub const KC_POWER: NonZeroU8 = NonZeroU8::new(0x66).unwrap();
/// Keypad =
pub const KC_KP_EQUAL: NonZeroU8 = NonZeroU8::new(0x67).unwrap();

/// Keyboard F13
pub const KC_F13: NonZeroU8 = NonZeroU8::new(0x68).unwrap();
/// Keyboard F14
pub const KC_F14: NonZeroU8 = NonZeroU8::new(0x69).unwrap();
/// Keyboard F15
pub const KC_F15: NonZeroU8 = NonZeroU8::new(0x6A).unwrap();
/// Keyboard F16
pub const KC_F16: NonZeroU8 = NonZeroU8::new(0x6B).unwrap();
/// Keyboard F17
pub const KC_F17: NonZeroU8 = NonZeroU8::new(0x6C).unwrap();
/// Keyboard F18
pub const KC_F18: NonZeroU8 = NonZeroU8::new(0x6D).unwrap();
/// Keyboard F19
pub const KC_F19: NonZeroU8 = NonZeroU8::new(0x6E).unwrap();
/// Keyboard F20
pub const KC_F20: NonZeroU8 = NonZeroU8::new(0x6F).unwrap();
/// Keyboard F21
pub const KC_F21: NonZeroU8 = NonZeroU8::new(0x70).unwrap();
/// Keyboard F22
pub const KC_F22: NonZeroU8 = NonZeroU8::new(0x71).unwrap();
/// Keyboard F23
pub const KC_F23: NonZeroU8 = NonZeroU8::new(0x72).unwrap();
/// Keyboard F24
pub const KC_F24: NonZeroU8 = NonZeroU8::new(0x73).unwrap();

/// Keyboard Execute
pub const KC_OPEN: NonZeroU8 = NonZeroU8::new(0x74).unwrap();
/// Keyboard Help
pub const KC_HELP: NonZeroU8 = NonZeroU8::new(0x75).unwrap();
/// Keyboard Menu
pub const KC_PROPS: NonZeroU8 = NonZeroU8::new(0x76).unwrap();
/// Keyboard Select
pub const KC_FRONT: NonZeroU8 = NonZeroU8::new(0x77).unwrap();
/// Keyboard Stop
pub const KC_STOP: NonZeroU8 = NonZeroU8::new(0x78).unwrap();
/// Keyboard Again
pub const KC_AGAIN: NonZeroU8 = NonZeroU8::new(0x79).unwrap();
/// Keyboard Undo
pub const KC_UNDO: NonZeroU8 = NonZeroU8::new(0x7A).unwrap();
/// Keyboard Cut
pub const KC_CUT: NonZeroU8 = NonZeroU8::new(0x7B).unwrap();
/// Keyboard Copy
pub const KC_COPY: NonZeroU8 = NonZeroU8::new(0x7C).unwrap();
/// Keyboard Paste
pub const KC_PASTE: NonZeroU8 = NonZeroU8::new(0x7D).unwrap();
/// Keyboard Find
pub const KC_FIND: NonZeroU8 = NonZeroU8::new(0x7E).unwrap();
/// Keyboard Mute
pub const KC_MUTE: NonZeroU8 = NonZeroU8::new(0x7F).unwrap();
/// Keyboard Volume Up
pub const KC_VOLUMEUP: NonZeroU8 = NonZeroU8::new(0x80).unwrap();
/// Keyboard Volume Down
pub const KC_VOLUMEDOWN: NonZeroU8 = NonZeroU8::new(0x81).unwrap();

// 0x82  Keyboard Locking Caps Lock
// 0x83  Keyboard Locking Num Lock
// 0x84  Keyboard Locking Scroll Lock

/// Keypad Comma
pub const KC_KP_COMMA: NonZeroU8 = NonZeroU8::new(0x85).unwrap();
// 0x86  Keypad Equal Sign

/// Keyboard International1
pub const KC_RO: NonZeroU8 = NonZeroU8::new(0x87).unwrap();
/// Keyboard International2
pub const KC_KATAKANAHIRAGANA: NonZeroU8 = NonZeroU8::new(0x88).unwrap();
/// Keyboard International3
pub const KC_YEN: NonZeroU8 = NonZeroU8::new(0x89).unwrap();
/// Keyboard International4
pub const KC_HENKAN: NonZeroU8 = NonZeroU8::new(0x8A).unwrap();
/// Keyboard International5
pub const KC_MUHENKAN: NonZeroU8 = NonZeroU8::new(0x8B).unwrap();
/// Keyboard International6
pub const KC_KPJPCOMMA: NonZeroU8 = NonZeroU8::new(0x8C).unwrap();
// 0x8d  Keyboard International7
// 0x8e  Keyboard International8
// 0x8f  Keyboard International9
/// Keyboard LANG1
pub const KC_HANGEUL: NonZeroU8 = NonZeroU8::new(0x90).unwrap();
/// Keyboard LANG2
pub const KC_HANJA: NonZeroU8 = NonZeroU8::new(0x91).unwrap();
/// Keyboard LANG3
pub const KC_KATAKANA: NonZeroU8 = NonZeroU8::new(0x92).unwrap();
/// Keyboard LANG4
pub const KC_HIRAGANA: NonZeroU8 = NonZeroU8::new(0x93).unwrap();
/// Keyboard LANG5
pub const KC_ZENKAKUHANKAKU: NonZeroU8 = NonZeroU8::new(0x94).unwrap();
// 0x95  Keyboard LANG6
// 0x96  Keyboard LANG7
// 0x97  Keyboard LANG8
// 0x98  Keyboard LANG9

// 0x99  Keyboard Alternate Erase
// 0x9a  Keyboard SysReq/Attention
// 0x9b  Keyboard Cancel
// 0x9c  Keyboard Clear
// 0x9d  Keyboard Prior
// 0x9e  Keyboard Return
// 0x9f  Keyboard Separator
// 0xa0  Keyboard Out
// 0xa1  Keyboard Oper
// 0xa2  Keyboard Clear/Again
// 0xa3  Keyboard CrSel/Props
// 0xa4  Keyboard ExSel

// 0xb0  Keypad 00
// 0xb1  Keypad 000
// 0xb2  Thousands Separator
// 0xb3  Decimal Separator
// 0xb4  Currency Unit
// 0xb5  Currency Sub-unit
/// Keypad (
pub const KC_KP_LEFTPAREN: NonZeroU8 = NonZeroU8::new(0xB6).unwrap();
/// Keypad )
pub const KC_KP_RIGHTPAREN: NonZeroU8 = NonZeroU8::new(0xB7).unwrap();
// 0xb8  Keypad {
// 0xb9  Keypad }
// 0xba  Keypad Tab
// 0xbb  Keypad Backspace
// 0xbc  Keypad A
// 0xbd  Keypad B
// 0xbe  Keypad C
// 0xbf  Keypad D
// 0xc0  Keypad E
// 0xc1  Keypad F
// 0xc2  Keypad XOR
// 0xc3  Keypad ^
// 0xc4  Keypad %
// 0xc5  Keypad <
// 0xc6  Keypad >
// 0xc7  Keypad &
// 0xc8  Keypad &&
// 0xc9  Keypad |
// 0xca  Keypad ||
// 0xcb  Keypad :
// 0xcc  Keypad #
// 0xcd  Keypad Space
// 0xce  Keypad @
// 0xcf  Keypad !
// 0xd0  Keypad Memory Store
// 0xd1  Keypad Memory Recall
// 0xd2  Keypad Memory Clear
// 0xd3  Keypad Memory Add
// 0xd4  Keypad Memory Subtract
// 0xd5  Keypad Memory Multiply
// 0xd6  Keypad Memory Divide
// 0xd7  Keypad +/-
// 0xd8  Keypad Clear
// 0xd9  Keypad Clear Entry
// 0xda  Keypad Binary
// 0xdb  Keypad Octal
// 0xdc  Keypad Decimal
// 0xdd  Keypad Hexadecimal

/// Keyboard Left Control
pub const KC_LEFTCTRL: NonZeroU8 = NonZeroU8::new(0xE0).unwrap();
/// Keyboard Left Shift
pub const KC_LEFTSHIFT: NonZeroU8 = NonZeroU8::new(0xE1).unwrap();
/// Keyboard Left Alt
pub const KC_LEFTALT: NonZeroU8 = NonZeroU8::new(0xE2).unwrap();
/// Keyboard Left GUI
pub const KC_LEFTMETA: NonZeroU8 = NonZeroU8::new(0xE3).unwrap();
/// Keyboard Right Control
pub const KC_RIGHTCTRL: NonZeroU8 = NonZeroU8::new(0xE4).unwrap();
/// Keyboard Right Shift
pub const KC_RIGHTSHIFT: NonZeroU8 = NonZeroU8::new(0xE5).unwrap();
/// Keyboard Right Alt
pub const KC_RIGHTALT: NonZeroU8 = NonZeroU8::new(0xE6).unwrap();
/// Keyboard Right GUI
pub const KC_RIGHTMETA: NonZeroU8 = NonZeroU8::new(0xE7).unwrap();

pub const KC_M_PLAYPAUSE: NonZeroU8 = NonZeroU8::new(0xE8).unwrap();
pub const KC_M_STOPCD: NonZeroU8 = NonZeroU8::new(0xE9).unwrap();
pub const KC_M_PREVIOUSSONG: NonZeroU8 = NonZeroU8::new(0xEA).unwrap();
pub const KC_M_NEXTSONG: NonZeroU8 = NonZeroU8::new(0xEB).unwrap();
pub const KC_M_EJECTCD: NonZeroU8 = NonZeroU8::new(0xEC).unwrap();
pub const KC_M_VOLUMEUP: NonZeroU8 = NonZeroU8::new(0xED).unwrap();
pub const KC_M_VOLUMEDOWN: NonZeroU8 = NonZeroU8::new(0xEE).unwrap();
pub const KC_M_MUTE: NonZeroU8 = NonZeroU8::new(0xEF).unwrap();
pub const KC_M_WWW: NonZeroU8 = NonZeroU8::new(0xF0).unwrap();
pub const KC_M_BACK: NonZeroU8 = NonZeroU8::new(0xF1).unwrap();
pub const KC_M_FORWARD: NonZeroU8 = NonZeroU8::new(0xF2).unwrap();
pub const KC_M_STOP: NonZeroU8 = NonZeroU8::new(0xF3).unwrap();
pub const KC_M_FIND: NonZeroU8 = NonZeroU8::new(0xF4).unwrap();
pub const KC_M_SCROLLUP: NonZeroU8 = NonZeroU8::new(0xF5).unwrap();
pub const KC_M_SCROLLDOWN: NonZeroU8 = NonZeroU8::new(0xF6).unwrap();
pub const KC_M_EDIT: NonZeroU8 = NonZeroU8::new(0xF7).unwrap();
pub const KC_M_SLEEP: NonZeroU8 = NonZeroU8::new(0xF8).unwrap();
pub const KC_M_COFFEE: NonZeroU8 = NonZeroU8::new(0xF9).unwrap();
pub const KC_M_REFRESH: NonZeroU8 = NonZeroU8::new(0xFA).unwrap();
pub const KC_M_CALC: NonZeroU8 = NonZeroU8::new(0xFB).unwrap();
