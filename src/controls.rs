use bevy::prelude::*;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[derive(Component, Debug, Deref, DerefMut, Clone, Copy)]
pub struct Key(pub KeyCode);

impl Serialize for Key {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{:?}", **self))
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct KeyCodeVisitor;

        impl Visitor<'_> for KeyCodeVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u32")
            }

            fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
                Ok(Key(match v {
                    0 => KeyCode::Digit1,
                    1 => KeyCode::Digit2,
                    2 => KeyCode::Digit3,
                    3 => KeyCode::Digit4,
                    4 => KeyCode::Digit5,
                    5 => KeyCode::Digit6,
                    6 => KeyCode::Digit7,
                    7 => KeyCode::Digit8,
                    8 => KeyCode::Digit9,
                    9 => KeyCode::Digit0,
                    10 => KeyCode::KeyA,
                    11 => KeyCode::KeyB,
                    12 => KeyCode::KeyC,
                    13 => KeyCode::KeyD,
                    14 => KeyCode::KeyE,
                    15 => KeyCode::KeyF,
                    16 => KeyCode::KeyG,
                    17 => KeyCode::KeyH,
                    18 => KeyCode::KeyI,
                    19 => KeyCode::KeyJ,
                    20 => KeyCode::KeyK,
                    21 => KeyCode::KeyL,
                    22 => KeyCode::KeyM,
                    23 => KeyCode::KeyN,
                    24 => KeyCode::KeyO,
                    25 => KeyCode::KeyP,
                    26 => KeyCode::KeyQ,
                    27 => KeyCode::KeyR,
                    28 => KeyCode::KeyS,
                    29 => KeyCode::KeyT,
                    30 => KeyCode::KeyU,
                    31 => KeyCode::KeyV,
                    32 => KeyCode::KeyW,
                    33 => KeyCode::KeyX,
                    34 => KeyCode::KeyY,
                    35 => KeyCode::KeyZ,
                    36 => KeyCode::Escape,
                    37 => KeyCode::F1,
                    38 => KeyCode::F2,
                    39 => KeyCode::F3,
                    40 => KeyCode::F4,
                    41 => KeyCode::F5,
                    42 => KeyCode::F6,
                    43 => KeyCode::F7,
                    44 => KeyCode::F8,
                    45 => KeyCode::F9,
                    46 => KeyCode::F10,
                    47 => KeyCode::F11,
                    48 => KeyCode::F12,
                    49 => KeyCode::F13,
                    50 => KeyCode::F14,
                    51 => KeyCode::F15,
                    52 => KeyCode::F16,
                    53 => KeyCode::F17,
                    54 => KeyCode::F18,
                    55 => KeyCode::F19,
                    56 => KeyCode::F20,
                    57 => KeyCode::F21,
                    58 => KeyCode::F22,
                    59 => KeyCode::F23,
                    60 => KeyCode::F24,
                    // 61 => KeyCode::Snapshot,
                    62 => KeyCode::ScrollLock,
                    63 => KeyCode::Pause,
                    64 => KeyCode::Insert,
                    65 => KeyCode::Home,
                    66 => KeyCode::Delete,
                    67 => KeyCode::End,
                    68 => KeyCode::PageDown,
                    69 => KeyCode::PageUp,
                    70 => KeyCode::ArrowLeft,
                    71 => KeyCode::ArrowUp,
                    72 => KeyCode::ArrowRight,
                    73 => KeyCode::ArrowDown,
                    74 => KeyCode::Backspace,
                    75 => KeyCode::Enter,
                    76 => KeyCode::Space,
                    // 77 => KeyCode::Compose,
                    // 78 => KeyCode::Caret,
                    79 => KeyCode::NumLock,
                    80 => KeyCode::Numpad0,
                    81 => KeyCode::Numpad1,
                    82 => KeyCode::Numpad2,
                    83 => KeyCode::Numpad3,
                    84 => KeyCode::Numpad4,
                    85 => KeyCode::Numpad5,
                    86 => KeyCode::Numpad6,
                    87 => KeyCode::Numpad7,
                    88 => KeyCode::Numpad8,
                    89 => KeyCode::Numpad9,
                    // 90 => KeyCode::AbntC1,
                    // 91 => KeyCode::AbntC2,
                    92 => KeyCode::NumpadAdd,
                    // 93 => KeyCode::Apostrophe,
                    // 94 => KeyCode::Apps,
                    // 95 => KeyCode::Asterisk,
                    // 96 => KeyCode::Plus,
                    // 97 => KeyCode::At,
                    // 98 => KeyCode::Ax,
                    99 => KeyCode::Backslash,
                    100 => KeyCode::LaunchApp2,
                    // 101 => KeyCode::Capital,
                    102 => KeyCode::Semicolon,
                    103 => KeyCode::Comma,
                    104 => KeyCode::Convert,
                    105 => KeyCode::NumpadDecimal,
                    106 => KeyCode::NumpadDivide,
                    107 => KeyCode::Equal,
                    // 108 => KeyCode::Grave,
                    109 => KeyCode::KanaMode,
                    // 110 => KeyCode::Kanji,
                    111 => KeyCode::AltLeft,
                    112 => KeyCode::BracketLeft,
                    113 => KeyCode::ControlLeft,
                    114 => KeyCode::ShiftLeft,
                    115 => KeyCode::SuperLeft,
                    116 => KeyCode::LaunchMail,
                    117 => KeyCode::MediaSelect,
                    118 => KeyCode::MediaStop,
                    119 => KeyCode::Minus,
                    120 => KeyCode::NumpadMultiply,
                    121 => KeyCode::AudioVolumeMute,
                    122 => KeyCode::LaunchApp1,
                    // 123 => KeyCode::NavigateForward,
                    // 124 => KeyCode::NavigateBackward,
                    125 => KeyCode::MediaTrackNext,
                    126 => KeyCode::NonConvert,
                    127 => KeyCode::NumpadComma,
                    128 => KeyCode::NumpadEnter,
                    129 => KeyCode::NumpadEqual,
                    // 130 => KeyCode::Oem102,
                    131 => KeyCode::Period,
                    132 => KeyCode::MediaPlayPause,
                    133 => KeyCode::Power,
                    134 => KeyCode::MediaTrackPrevious,
                    135 => KeyCode::AltRight,
                    136 => KeyCode::BracketRight,
                    137 => KeyCode::ControlRight,
                    138 => KeyCode::ShiftRight,
                    139 => KeyCode::SuperRight,
                    140 => KeyCode::Semicolon,
                    141 => KeyCode::Slash,
                    142 => KeyCode::Sleep,
                    143 => KeyCode::MediaStop,
                    144 => KeyCode::NumpadSubtract,
                    145 => KeyCode::PrintScreen,
                    146 => KeyCode::Tab,
                    // 147 => KeyCode::Underline,
                    // 148 => KeyCode::Unlabeled,
                    149 => KeyCode::AudioVolumeDown,
                    150 => KeyCode::AudioVolumeUp,
                    151 => KeyCode::WakeUp,
                    152 => KeyCode::BrowserBack,
                    153 => KeyCode::BrowserFavorites,
                    154 => KeyCode::BrowserForward,
                    155 => KeyCode::BrowserHome,
                    156 => KeyCode::BrowserRefresh,
                    157 => KeyCode::BrowserSearch,
                    158 => KeyCode::BrowserStop,
                    159 => KeyCode::IntlYen,
                    160 => KeyCode::Copy,
                    161 => KeyCode::Paste,
                    162 => KeyCode::Cut,
                    v => unreachable!("Deserialized unknown KeyCode {v}"),
                }))
            }
        }

        deserializer.deserialize_u32(KeyCodeVisitor)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Resource)]
pub struct KeyBindings {
    pub up: [Key; 3],
    pub down: [Key; 2],
    pub left: [Key; 3],
    pub right: [Key; 2],
    pub interact: [Key; 2],
}

impl KeyBindings {
    pub fn up(&self) -> [KeyCode; 2] {
        [*self.up[0], *self.up[1]]
    }

    pub fn down(&self) -> [KeyCode; 2] {
        [*self.down[0], *self.down[1]]
    }

    pub fn left(&self) -> [KeyCode; 2] {
        [*self.left[0], *self.left[1]]
    }

    pub fn right(&self) -> [KeyCode; 2] {
        [*self.right[0], *self.right[1]]
    }

    pub fn interact(&self) -> [KeyCode; 2] {
        [*self.interact[0], *self.interact[1]]
    }
}

pub fn save_key_bindings(_key_bindings: Res<KeyBindings>) {}

// pub fn load_key_bindings() -> KeyBindings {
//     ron::de::from_bytes(include_bytes!("data/dialogs.ron")).unwrap()
// }
