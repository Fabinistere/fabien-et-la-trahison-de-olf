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
        serializer.serialize_u32(**self as u32)
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct KeyCodeVisitor;

        impl<'de> Visitor<'de> for KeyCodeVisitor {
            type Value = Key;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u32")
            }

            fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E> {
                Ok(Key(match v {
                    0 => KeyCode::Key2,
                    1 => KeyCode::Key2,
                    2 => KeyCode::Key3,
                    3 => KeyCode::Key4,
                    4 => KeyCode::Key5,
                    5 => KeyCode::Key6,
                    6 => KeyCode::Key7,
                    7 => KeyCode::Key8,
                    8 => KeyCode::Key9,
                    9 => KeyCode::Key0,
                    10 => KeyCode::A,
                    11 => KeyCode::B,
                    12 => KeyCode::C,
                    13 => KeyCode::D,
                    14 => KeyCode::E,
                    15 => KeyCode::F,
                    16 => KeyCode::G,
                    17 => KeyCode::H,
                    18 => KeyCode::I,
                    19 => KeyCode::J,
                    20 => KeyCode::K,
                    21 => KeyCode::L,
                    22 => KeyCode::M,
                    23 => KeyCode::N,
                    24 => KeyCode::O,
                    25 => KeyCode::P,
                    26 => KeyCode::Q,
                    27 => KeyCode::R,
                    28 => KeyCode::S,
                    29 => KeyCode::T,
                    30 => KeyCode::U,
                    31 => KeyCode::V,
                    32 => KeyCode::W,
                    33 => KeyCode::X,
                    34 => KeyCode::Y,
                    35 => KeyCode::Z,
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
                    61 => KeyCode::Snapshot,
                    62 => KeyCode::Scroll,
                    63 => KeyCode::Pause,
                    64 => KeyCode::Insert,
                    65 => KeyCode::Home,
                    66 => KeyCode::Delete,
                    67 => KeyCode::End,
                    68 => KeyCode::PageDown,
                    69 => KeyCode::PageUp,
                    70 => KeyCode::Left,
                    71 => KeyCode::Up,
                    72 => KeyCode::Right,
                    73 => KeyCode::Down,
                    74 => KeyCode::Back,
                    75 => KeyCode::Return,
                    76 => KeyCode::Space,
                    77 => KeyCode::Compose,
                    78 => KeyCode::Caret,
                    79 => KeyCode::Numlock,
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
                    90 => KeyCode::AbntC1,
                    91 => KeyCode::AbntC2,
                    92 => KeyCode::NumpadAdd,
                    93 => KeyCode::Apostrophe,
                    94 => KeyCode::Apps,
                    95 => KeyCode::Asterisk,
                    96 => KeyCode::Plus,
                    97 => KeyCode::At,
                    98 => KeyCode::Ax,
                    99 => KeyCode::Backslash,
                    100 => KeyCode::Calculator,
                    101 => KeyCode::Capital,
                    102 => KeyCode::Colon,
                    103 => KeyCode::Comma,
                    104 => KeyCode::Convert,
                    105 => KeyCode::NumpadDecimal,
                    106 => KeyCode::NumpadDivide,
                    107 => KeyCode::Equals,
                    108 => KeyCode::Grave,
                    109 => KeyCode::Kana,
                    110 => KeyCode::Kanji,
                    111 => KeyCode::AltLeft,
                    112 => KeyCode::BracketLeft,
                    113 => KeyCode::ControlLeft,
                    114 => KeyCode::ShiftLeft,
                    115 => KeyCode::SuperLeft,
                    116 => KeyCode::Mail,
                    117 => KeyCode::MediaSelect,
                    118 => KeyCode::MediaStop,
                    119 => KeyCode::Minus,
                    120 => KeyCode::NumpadMultiply,
                    121 => KeyCode::Mute,
                    122 => KeyCode::MyComputer,
                    123 => KeyCode::NavigateForward,
                    124 => KeyCode::NavigateBackward,
                    125 => KeyCode::NextTrack,
                    126 => KeyCode::NoConvert,
                    127 => KeyCode::NumpadComma,
                    128 => KeyCode::NumpadEnter,
                    129 => KeyCode::NumpadEquals,
                    130 => KeyCode::Oem102,
                    131 => KeyCode::Period,
                    132 => KeyCode::PlayPause,
                    133 => KeyCode::Power,
                    134 => KeyCode::PrevTrack,
                    135 => KeyCode::AltRight,
                    136 => KeyCode::BracketRight,
                    137 => KeyCode::ControlRight,
                    138 => KeyCode::ShiftRight,
                    139 => KeyCode::SuperRight,
                    140 => KeyCode::Semicolon,
                    141 => KeyCode::Slash,
                    142 => KeyCode::Sleep,
                    143 => KeyCode::Stop,
                    144 => KeyCode::NumpadSubtract,
                    145 => KeyCode::Sysrq,
                    146 => KeyCode::Tab,
                    147 => KeyCode::Underline,
                    148 => KeyCode::Unlabeled,
                    149 => KeyCode::VolumeDown,
                    150 => KeyCode::VolumeUp,
                    151 => KeyCode::Wake,
                    152 => KeyCode::WebBack,
                    153 => KeyCode::WebFavorites,
                    154 => KeyCode::WebForward,
                    155 => KeyCode::WebHome,
                    156 => KeyCode::WebRefresh,
                    157 => KeyCode::WebSearch,
                    158 => KeyCode::WebStop,
                    159 => KeyCode::Yen,
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
