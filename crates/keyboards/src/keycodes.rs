#![expect(clippy::doc_markdown)]

//! USB HID Keyboard modifier masks and scan codes as per USB spec.

use core::num::NonZeroU8;

// -- Modifier masks --
//
// Used for the first byte in the HID report.

pub const KEY_MOD_LCTRL: NonZeroU8 = NonZeroU8::new(0x01).unwrap();
pub const KEY_MOD_LSHIFT: NonZeroU8 = NonZeroU8::new(0x02).unwrap();
pub const KEY_MOD_LALT: NonZeroU8 = NonZeroU8::new(0x04).unwrap();
pub const KEY_MOD_LMETA: NonZeroU8 = NonZeroU8::new(0x08).unwrap();
pub const KEY_MOD_RCTRL: NonZeroU8 = NonZeroU8::new(0x10).unwrap();
pub const KEY_MOD_RSHIFT: NonZeroU8 = NonZeroU8::new(0x20).unwrap();
pub const KEY_MOD_RALT: NonZeroU8 = NonZeroU8::new(0x40).unwrap();
pub const KEY_MOD_RMETA: NonZeroU8 = NonZeroU8::new(0x80).unwrap();

/// The Second byte in the report, which is reserved.
pub const RESERVED: u8 = 0x00;

// -- Scan codes. --
//
// Last N slots in the HID report (usually 6).
// 0x00 if no key pressed.
//
// If more than N keys are pressed, the HID reports
// KEY_ERR_OVF in all slots to indicate this condition.

/// No key pressed
pub const KEY_NONE: u8 = 0x00;
/// Keyboard Error Roll Over
///
/// Used for all slots if too many keys are pressed ("Phantom key")
pub const KEY_ERR_OVF: NonZeroU8 = NonZeroU8::new(0x01).unwrap();

// 0x02 //  Keyboard POST Fail
// 0x03 //  Keyboard Error Undefined

/// Keyboard a and A
pub const KEY_A: NonZeroU8 = NonZeroU8::new(0x04).unwrap();
/// Keyboard b and B
pub const KEY_B: NonZeroU8 = NonZeroU8::new(0x05).unwrap();
/// Keyboard c and C
pub const KEY_C: NonZeroU8 = NonZeroU8::new(0x06).unwrap();
/// Keyboard d and D
pub const KEY_D: NonZeroU8 = NonZeroU8::new(0x07).unwrap();
/// Keyboard e and E
pub const KEY_E: NonZeroU8 = NonZeroU8::new(0x08).unwrap();
/// Keyboard f and F
pub const KEY_F: NonZeroU8 = NonZeroU8::new(0x09).unwrap();
/// Keyboard g and G
pub const KEY_G: NonZeroU8 = NonZeroU8::new(0x0a).unwrap();
/// Keyboard h and H
pub const KEY_H: NonZeroU8 = NonZeroU8::new(0x0b).unwrap();
/// Keyboard i and I
pub const KEY_I: NonZeroU8 = NonZeroU8::new(0x0c).unwrap();
/// Keyboard j and J
pub const KEY_J: NonZeroU8 = NonZeroU8::new(0x0d).unwrap();
/// Keyboard k and K
pub const KEY_K: NonZeroU8 = NonZeroU8::new(0x0e).unwrap();
/// Keyboard l and L
pub const KEY_L: NonZeroU8 = NonZeroU8::new(0x0f).unwrap();
/// Keyboard m and M
pub const KEY_M: NonZeroU8 = NonZeroU8::new(0x10).unwrap();
/// Keyboard n and N
pub const KEY_N: NonZeroU8 = NonZeroU8::new(0x11).unwrap();
/// Keyboard o and O
pub const KEY_O: NonZeroU8 = NonZeroU8::new(0x12).unwrap();
/// Keyboard p and P
pub const KEY_P: NonZeroU8 = NonZeroU8::new(0x13).unwrap();
/// Keyboard q and Q
pub const KEY_Q: NonZeroU8 = NonZeroU8::new(0x14).unwrap();
/// Keyboard r and R
pub const KEY_R: NonZeroU8 = NonZeroU8::new(0x15).unwrap();
/// Keyboard s and S
pub const KEY_S: NonZeroU8 = NonZeroU8::new(0x16).unwrap();
/// Keyboard t and T
pub const KEY_T: NonZeroU8 = NonZeroU8::new(0x17).unwrap();
/// Keyboard u and U
pub const KEY_U: NonZeroU8 = NonZeroU8::new(0x18).unwrap();
/// Keyboard v and V
pub const KEY_V: NonZeroU8 = NonZeroU8::new(0x19).unwrap();
/// Keyboard w and W
pub const KEY_W: NonZeroU8 = NonZeroU8::new(0x1a).unwrap();
/// Keyboard x and X
pub const KEY_X: NonZeroU8 = NonZeroU8::new(0x1b).unwrap();
/// Keyboard y and Y
pub const KEY_Y: NonZeroU8 = NonZeroU8::new(0x1c).unwrap();
/// Keyboard z and Z
pub const KEY_Z: NonZeroU8 = NonZeroU8::new(0x1d).unwrap();

/// Keyboard 1 and !
pub const KEY_1: NonZeroU8 = NonZeroU8::new(0x1e).unwrap();
/// Keyboard 2 and @
pub const KEY_2: NonZeroU8 = NonZeroU8::new(0x1f).unwrap();
/// Keyboard 3 and #
pub const KEY_3: NonZeroU8 = NonZeroU8::new(0x20).unwrap();
/// Keyboard 4 and $
pub const KEY_4: NonZeroU8 = NonZeroU8::new(0x21).unwrap();
/// Keyboard 5 and %
pub const KEY_5: NonZeroU8 = NonZeroU8::new(0x22).unwrap();
/// Keyboard 6 and ^
pub const KEY_6: NonZeroU8 = NonZeroU8::new(0x23).unwrap();
/// Keyboard 7 and &
pub const KEY_7: NonZeroU8 = NonZeroU8::new(0x24).unwrap();
/// Keyboard 8 and *
pub const KEY_8: NonZeroU8 = NonZeroU8::new(0x25).unwrap();
/// Keyboard 9 and (
pub const KEY_9: NonZeroU8 = NonZeroU8::new(0x26).unwrap();
/// Keyboard 0 and )
pub const KEY_0: NonZeroU8 = NonZeroU8::new(0x27).unwrap();

/// Keyboard Return (ENTER)
pub const KEY_ENTER: NonZeroU8 = NonZeroU8::new(0x28).unwrap();
/// Keyboard ESCAPE
pub const KEY_ESC: NonZeroU8 = NonZeroU8::new(0x29).unwrap();
/// Keyboard DELETE (Backspace)
pub const KEY_BACKSPACE: NonZeroU8 = NonZeroU8::new(0x2a).unwrap();
/// Keyboard Tab
pub const KEY_TAB: NonZeroU8 = NonZeroU8::new(0x2b).unwrap();
/// Keyboard Spacebar
pub const KEY_SPACE: NonZeroU8 = NonZeroU8::new(0x2c).unwrap();
/// Keyboard - and _
pub const KEY_MINUS: NonZeroU8 = NonZeroU8::new(0x2d).unwrap();
/// Keyboard = and +
pub const KEY_EQUAL: NonZeroU8 = NonZeroU8::new(0x2e).unwrap();
/// Keyboard [ and {
pub const KEY_LEFTBRACE: NonZeroU8 = NonZeroU8::new(0x2f).unwrap();
/// Keyboard ] and }
pub const KEY_RIGHTBRACE: NonZeroU8 = NonZeroU8::new(0x30).unwrap();
/// Keyboard \ and |
pub const KEY_BACKSLASH: NonZeroU8 = NonZeroU8::new(0x31).unwrap();
/// Keyboard Non-US # and ~
pub const KEY_HASHTILDE: NonZeroU8 = NonZeroU8::new(0x32).unwrap();
/// Keyboard ; and :
pub const KEY_SEMICOLON: NonZeroU8 = NonZeroU8::new(0x33).unwrap();
/// Keyboard ' and "
pub const KEY_APOSTROPHE: NonZeroU8 = NonZeroU8::new(0x34).unwrap();
/// Keyboard ` and ~
pub const KEY_GRAVE: NonZeroU8 = NonZeroU8::new(0x35).unwrap();
/// Keyboard , and <
pub const KEY_COMMA: NonZeroU8 = NonZeroU8::new(0x36).unwrap();
/// Keyboard . and >
pub const KEY_DOT: NonZeroU8 = NonZeroU8::new(0x37).unwrap();
/// Keyboard / and ?
pub const KEY_SLASH: NonZeroU8 = NonZeroU8::new(0x38).unwrap();
/// Keyboard Caps Lock
pub const KEY_CAPSLOCK: NonZeroU8 = NonZeroU8::new(0x39).unwrap();

/// Keyboard F1
pub const KEY_F1: NonZeroU8 = NonZeroU8::new(0x3a).unwrap();
/// Keyboard F2
pub const KEY_F2: NonZeroU8 = NonZeroU8::new(0x3b).unwrap();
/// Keyboard F3
pub const KEY_F3: NonZeroU8 = NonZeroU8::new(0x3c).unwrap();
/// Keyboard F4
pub const KEY_F4: NonZeroU8 = NonZeroU8::new(0x3d).unwrap();
/// Keyboard F5
pub const KEY_F5: NonZeroU8 = NonZeroU8::new(0x3e).unwrap();
/// Keyboard F6
pub const KEY_F6: NonZeroU8 = NonZeroU8::new(0x3f).unwrap();
/// Keyboard F7
pub const KEY_F7: NonZeroU8 = NonZeroU8::new(0x40).unwrap();
/// Keyboard F8
pub const KEY_F8: NonZeroU8 = NonZeroU8::new(0x41).unwrap();
/// Keyboard F9
pub const KEY_F9: NonZeroU8 = NonZeroU8::new(0x42).unwrap();
/// Keyboard F10
pub const KEY_F10: NonZeroU8 = NonZeroU8::new(0x43).unwrap();
/// Keyboard F11
pub const KEY_F11: NonZeroU8 = NonZeroU8::new(0x44).unwrap();
/// Keyboard F12
pub const KEY_F12: NonZeroU8 = NonZeroU8::new(0x45).unwrap();

/// Keyboard Print Screen
pub const KEY_SYSRQ: NonZeroU8 = NonZeroU8::new(0x46).unwrap();
/// Keyboard Scroll Lock
pub const KEY_SCROLLLOCK: NonZeroU8 = NonZeroU8::new(0x47).unwrap();
/// Keyboard Pause
pub const KEY_PAUSE: NonZeroU8 = NonZeroU8::new(0x48).unwrap();
/// Keyboard Insert
pub const KEY_INSERT: NonZeroU8 = NonZeroU8::new(0x49).unwrap();
/// Keyboard Home
pub const KEY_HOME: NonZeroU8 = NonZeroU8::new(0x4a).unwrap();
/// Keyboard Page Up
pub const KEY_PAGEUP: NonZeroU8 = NonZeroU8::new(0x4b).unwrap();
/// Keyboard Delete Forward
pub const KEY_DELETE: NonZeroU8 = NonZeroU8::new(0x4c).unwrap();
/// Keyboard End
pub const KEY_END: NonZeroU8 = NonZeroU8::new(0x4d).unwrap();
/// Keyboard Page Down
pub const KEY_PAGEDOWN: NonZeroU8 = NonZeroU8::new(0x4e).unwrap();
/// Keyboard Right Arrow
pub const KEY_RIGHT: NonZeroU8 = NonZeroU8::new(0x4f).unwrap();
/// Keyboard Left Arrow
pub const KEY_LEFT: NonZeroU8 = NonZeroU8::new(0x50).unwrap();
/// Keyboard Down Arrow
pub const KEY_DOWN: NonZeroU8 = NonZeroU8::new(0x51).unwrap();
/// Keyboard Up Arrow
pub const KEY_UP: NonZeroU8 = NonZeroU8::new(0x52).unwrap();

/// Keyboard Num Lock and Clear
pub const KEY_NUMLOCK: NonZeroU8 = NonZeroU8::new(0x53).unwrap();
/// Keypad /
pub const KEY_KP_SLASH: NonZeroU8 = NonZeroU8::new(0x54).unwrap();
/// Keypad *
pub const KEY_KP_ASTERISK: NonZeroU8 = NonZeroU8::new(0x55).unwrap();
/// Keypad -
pub const KEY_KP_MINUS: NonZeroU8 = NonZeroU8::new(0x56).unwrap();
/// Keypad +
pub const KEY_KP_PLUS: NonZeroU8 = NonZeroU8::new(0x57).unwrap();
/// Keypad ENTER
pub const KEY_KP_ENTER: NonZeroU8 = NonZeroU8::new(0x58).unwrap();
/// Keypad 1 and End
pub const KEY_KP_1: NonZeroU8 = NonZeroU8::new(0x59).unwrap();
/// Keypad 2 and Down Arrow
pub const KEY_KP_2: NonZeroU8 = NonZeroU8::new(0x5a).unwrap();
/// Keypad 3 and PageDn
pub const KEY_KP_3: NonZeroU8 = NonZeroU8::new(0x5b).unwrap();
/// Keypad 4 and Left Arrow
pub const KEY_KP_4: NonZeroU8 = NonZeroU8::new(0x5c).unwrap();
/// Keypad 5
pub const KEY_KP_5: NonZeroU8 = NonZeroU8::new(0x5d).unwrap();
/// Keypad 6 and Right Arrow
pub const KEY_KP_6: NonZeroU8 = NonZeroU8::new(0x5e).unwrap();
/// Keypad 7 and Home
pub const KEY_KP_7: NonZeroU8 = NonZeroU8::new(0x5f).unwrap();
/// Keypad 8 and Up Arrow
pub const KEY_KP_8: NonZeroU8 = NonZeroU8::new(0x60).unwrap();
/// Keypad 9 and Page Up
pub const KEY_KP_9: NonZeroU8 = NonZeroU8::new(0x61).unwrap();
/// Keypad 0 and Insert
pub const KEY_KP_0: NonZeroU8 = NonZeroU8::new(0x62).unwrap();
/// Keypad . and Delete
pub const KEY_KP_DOT: NonZeroU8 = NonZeroU8::new(0x63).unwrap();

/// Keyboard Non-US \ and |
pub const KEY_102ND: NonZeroU8 = NonZeroU8::new(0x64).unwrap();
/// Keyboard Application
pub const KEY_COMPOSE: NonZeroU8 = NonZeroU8::new(0x65).unwrap();
/// Keyboard Power
pub const KEY_POWER: NonZeroU8 = NonZeroU8::new(0x66).unwrap();
/// Keypad =
pub const KEY_KP_EQUAL: NonZeroU8 = NonZeroU8::new(0x67).unwrap();

/// Keyboard F13
pub const KEY_F13: NonZeroU8 = NonZeroU8::new(0x68).unwrap();
/// Keyboard F14
pub const KEY_F14: NonZeroU8 = NonZeroU8::new(0x69).unwrap();
/// Keyboard F15
pub const KEY_F15: NonZeroU8 = NonZeroU8::new(0x6a).unwrap();
/// Keyboard F16
pub const KEY_F16: NonZeroU8 = NonZeroU8::new(0x6b).unwrap();
/// Keyboard F17
pub const KEY_F17: NonZeroU8 = NonZeroU8::new(0x6c).unwrap();
/// Keyboard F18
pub const KEY_F18: NonZeroU8 = NonZeroU8::new(0x6d).unwrap();
/// Keyboard F19
pub const KEY_F19: NonZeroU8 = NonZeroU8::new(0x6e).unwrap();
/// Keyboard F20
pub const KEY_F20: NonZeroU8 = NonZeroU8::new(0x6f).unwrap();
/// Keyboard F21
pub const KEY_F21: NonZeroU8 = NonZeroU8::new(0x70).unwrap();
/// Keyboard F22
pub const KEY_F22: NonZeroU8 = NonZeroU8::new(0x71).unwrap();
/// Keyboard F23
pub const KEY_F23: NonZeroU8 = NonZeroU8::new(0x72).unwrap();
/// Keyboard F24
pub const KEY_F24: NonZeroU8 = NonZeroU8::new(0x73).unwrap();

/// Keyboard Execute
pub const KEY_OPEN: NonZeroU8 = NonZeroU8::new(0x74).unwrap();
/// Keyboard Help
pub const KEY_HELP: NonZeroU8 = NonZeroU8::new(0x75).unwrap();
/// Keyboard Menu
pub const KEY_PROPS: NonZeroU8 = NonZeroU8::new(0x76).unwrap();
/// Keyboard Select
pub const KEY_FRONT: NonZeroU8 = NonZeroU8::new(0x77).unwrap();
/// Keyboard Stop
pub const KEY_STOP: NonZeroU8 = NonZeroU8::new(0x78).unwrap();
/// Keyboard Again
pub const KEY_AGAIN: NonZeroU8 = NonZeroU8::new(0x79).unwrap();
/// Keyboard Undo
pub const KEY_UNDO: NonZeroU8 = NonZeroU8::new(0x7a).unwrap();
/// Keyboard Cut
pub const KEY_CUT: NonZeroU8 = NonZeroU8::new(0x7b).unwrap();
/// Keyboard Copy
pub const KEY_COPY: NonZeroU8 = NonZeroU8::new(0x7c).unwrap();
/// Keyboard Paste
pub const KEY_PASTE: NonZeroU8 = NonZeroU8::new(0x7d).unwrap();
/// Keyboard Find
pub const KEY_FIND: NonZeroU8 = NonZeroU8::new(0x7e).unwrap();
/// Keyboard Mute
pub const KEY_MUTE: NonZeroU8 = NonZeroU8::new(0x7f).unwrap();
/// Keyboard Volume Up
pub const KEY_VOLUMEUP: NonZeroU8 = NonZeroU8::new(0x80).unwrap();
/// Keyboard Volume Down
pub const KEY_VOLUMEDOWN: NonZeroU8 = NonZeroU8::new(0x81).unwrap();

// 0x82  Keyboard Locking Caps Lock
// 0x83  Keyboard Locking Num Lock
// 0x84  Keyboard Locking Scroll Lock
/// Keypad Comma
pub const KEY_KPCOMMA: NonZeroU8 = NonZeroU8::new(0x85).unwrap();
// 0x86  Keypad Equal Sign

/// Keyboard International1
pub const KEY_RO: NonZeroU8 = NonZeroU8::new(0x87).unwrap();
/// Keyboard International2
pub const KEY_KATAKANAHIRAGANA: NonZeroU8 = NonZeroU8::new(0x88).unwrap();
/// Keyboard International3
pub const KEY_YEN: NonZeroU8 = NonZeroU8::new(0x89).unwrap();
/// Keyboard International4
pub const KEY_HENKAN: NonZeroU8 = NonZeroU8::new(0x8a).unwrap();
/// Keyboard International5
pub const KEY_MUHENKAN: NonZeroU8 = NonZeroU8::new(0x8b).unwrap();
/// Keyboard International6
pub const KEY_KPJPCOMMA: NonZeroU8 = NonZeroU8::new(0x8c).unwrap();
// 0x8d  Keyboard International7
// 0x8e  Keyboard International8
// 0x8f  Keyboard International9
/// Keyboard LANG1
pub const KEY_HANGEUL: NonZeroU8 = NonZeroU8::new(0x90).unwrap();
/// Keyboard LANG2
pub const KEY_HANJA: NonZeroU8 = NonZeroU8::new(0x91).unwrap();
/// Keyboard LANG3
pub const KEY_KATAKANA: NonZeroU8 = NonZeroU8::new(0x92).unwrap();
/// Keyboard LANG4
pub const KEY_HIRAGANA: NonZeroU8 = NonZeroU8::new(0x93).unwrap();
/// Keyboard LANG5
pub const KEY_ZENKAKUHANKAKU: NonZeroU8 = NonZeroU8::new(0x94).unwrap();
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
pub const KEY_KPLEFTPAREN: NonZeroU8 = NonZeroU8::new(0xb6).unwrap();
/// Keypad )
pub const KEY_KPRIGHTPAREN: NonZeroU8 = NonZeroU8::new(0xb7).unwrap();
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
pub const KEY_LEFTCTRL: NonZeroU8 = NonZeroU8::new(0xe0).unwrap();
/// Keyboard Left Shift
pub const KEY_LEFTSHIFT: NonZeroU8 = NonZeroU8::new(0xe1).unwrap();
/// Keyboard Left Alt
pub const KEY_LEFTALT: NonZeroU8 = NonZeroU8::new(0xe2).unwrap();
/// Keyboard Left GUI
pub const KEY_LEFTMETA: NonZeroU8 = NonZeroU8::new(0xe3).unwrap();
/// Keyboard Right Control
pub const KEY_RIGHTCTRL: NonZeroU8 = NonZeroU8::new(0xe4).unwrap();
/// Keyboard Right Shift
pub const KEY_RIGHTSHIFT: NonZeroU8 = NonZeroU8::new(0xe5).unwrap();
/// Keyboard Right Alt
pub const KEY_RIGHTALT: NonZeroU8 = NonZeroU8::new(0xe6).unwrap();
/// Keyboard Right GUI
pub const KEY_RIGHTMETA: NonZeroU8 = NonZeroU8::new(0xe7).unwrap();

pub const KEY_MEDIA_PLAYPAUSE: NonZeroU8 = NonZeroU8::new(0xe8).unwrap();
pub const KEY_MEDIA_STOPCD: NonZeroU8 = NonZeroU8::new(0xe9).unwrap();
pub const KEY_MEDIA_PREVIOUSSONG: NonZeroU8 = NonZeroU8::new(0xea).unwrap();
pub const KEY_MEDIA_NEXTSONG: NonZeroU8 = NonZeroU8::new(0xeb).unwrap();
pub const KEY_MEDIA_EJECTCD: NonZeroU8 = NonZeroU8::new(0xec).unwrap();
pub const KEY_MEDIA_VOLUMEUP: NonZeroU8 = NonZeroU8::new(0xed).unwrap();
pub const KEY_MEDIA_VOLUMEDOWN: NonZeroU8 = NonZeroU8::new(0xee).unwrap();
pub const KEY_MEDIA_MUTE: NonZeroU8 = NonZeroU8::new(0xef).unwrap();
pub const KEY_MEDIA_WWW: NonZeroU8 = NonZeroU8::new(0xf0).unwrap();
pub const KEY_MEDIA_BACK: NonZeroU8 = NonZeroU8::new(0xf1).unwrap();
pub const KEY_MEDIA_FORWARD: NonZeroU8 = NonZeroU8::new(0xf2).unwrap();
pub const KEY_MEDIA_STOP: NonZeroU8 = NonZeroU8::new(0xf3).unwrap();
pub const KEY_MEDIA_FIND: NonZeroU8 = NonZeroU8::new(0xf4).unwrap();
pub const KEY_MEDIA_SCROLLUP: NonZeroU8 = NonZeroU8::new(0xf5).unwrap();
pub const KEY_MEDIA_SCROLLDOWN: NonZeroU8 = NonZeroU8::new(0xf6).unwrap();
pub const KEY_MEDIA_EDIT: NonZeroU8 = NonZeroU8::new(0xf7).unwrap();
pub const KEY_MEDIA_SLEEP: NonZeroU8 = NonZeroU8::new(0xf8).unwrap();
pub const KEY_MEDIA_COFFEE: NonZeroU8 = NonZeroU8::new(0xf9).unwrap();
pub const KEY_MEDIA_REFRESH: NonZeroU8 = NonZeroU8::new(0xfa).unwrap();
pub const KEY_MEDIA_CALC: NonZeroU8 = NonZeroU8::new(0xfb).unwrap();
