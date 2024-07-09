use slint::SharedString;

#[derive(Clone)]
pub struct KeyValue {
    pub desc: SharedString,
    pub byte: u8
}

#[derive(Clone)]
pub struct KeyValues {
    pub keys: Vec<KeyValue>
}

impl slint::Model for KeyValues {
    type Data = SharedString;

    fn row_count(&self) -> usize {
        self.keys.len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.keys.get(row).map(|x| format!("{}", x.desc ).try_into().unwrap() )
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &()
    }
}


impl KeyValues {
    pub fn new() -> Self {
        Self {
            keys: vec![
                KeyValue {desc: "".into(), byte: 0x00},

                KeyValue {desc: "KEY_UP_ARROW".into(),      byte: 0xDA},
                KeyValue {desc: "KEY_DOWN_ARROW".into(),    byte: 0xD9},
                KeyValue {desc: "KEY_LEFT_ARROW".into(),    byte: 0xD8},
                KeyValue {desc: "KEY_RIGHT_ARROW".into(),   byte: 0xD7},
                KeyValue {desc: "KEY_BACKSPACE".into(),     byte: 0xB2},
                KeyValue {desc: "KEY_TAB".into(),           byte: 0xB3},
                KeyValue {desc: "KEY_RETURN".into(),        byte: 0xB0},
                KeyValue {desc: "KEY_MENU".into(),          byte: 0xED}, // "Keyboard Application" in USB standard
                KeyValue {desc: "KEY_ESC".into(),           byte: 0xB1},
                KeyValue {desc: "KEY_INSERT".into(),        byte: 0xD1},
                KeyValue {desc: "KEY_DELETE".into(),        byte: 0xD4},
                KeyValue {desc: "KEY_PAGE_UP".into(),       byte: 0xD3},
                KeyValue {desc: "KEY_PAGE_DOWN".into(),     byte: 0xD6},
                KeyValue {desc: "KEY_HOME".into(),          byte: 0xD2},
                KeyValue {desc: "KEY_END".into(),           byte: 0xD5},
                KeyValue {desc: "KEY_CAPS_LOCK".into(),     byte: 0xC1},
                KeyValue {desc: "KEY_PRINT_SCREEN".into(),  byte: 0xCE}, // Print Screen / SysRq
                KeyValue {desc: "KEY_SCROLL_LOCK".into(),   byte: 0xCF},
                KeyValue {desc: "KEY_PAUSE".into(),         byte: 0xD0}, // Pause / Break

                // Numeric keypad
                KeyValue {desc: "KEY_NUM_LOCK".into(),      byte: 0xDB},
                KeyValue {desc: "KEY_KP_SLASH".into(),      byte: 0xDC},
                KeyValue {desc: "KEY_KP_ASTERISK".into(),   byte: 0xDD},
                KeyValue {desc: "KEY_KP_MINUS".into(),      byte: 0xDE},
                KeyValue {desc: "KEY_KP_PLUS".into(),       byte: 0xDF},
                KeyValue {desc: "KEY_KP_ENTER".into(),      byte: 0xE0},
                KeyValue {desc: "KEY_KP_1".into(),          byte: 0xE1},
                KeyValue {desc: "KEY_KP_2".into(),          byte: 0xE2},
                KeyValue {desc: "KEY_KP_3".into(),          byte: 0xE3},
                KeyValue {desc: "KEY_KP_4".into(),          byte: 0xE4},
                KeyValue {desc: "KEY_KP_5".into(),          byte: 0xE5},
                KeyValue {desc: "KEY_KP_6".into(),          byte: 0xE6},
                KeyValue {desc: "KEY_KP_7".into(),          byte: 0xE7},
                KeyValue {desc: "KEY_KP_8".into(),          byte: 0xE8},
                KeyValue {desc: "KEY_KP_9".into(),          byte: 0xE9},
                KeyValue {desc: "KEY_KP_0".into(),          byte: 0xEA},
                KeyValue {desc: "KEY_KP_DOT".into(),        byte: 0xEB},

                // Function keys
                KeyValue {desc: "KEY_F1".into(),            byte: 0xC2},
                KeyValue {desc: "KEY_F2".into(),            byte: 0xC3},
                KeyValue {desc: "KEY_F3".into(),            byte: 0xC4},
                KeyValue {desc: "KEY_F4".into(),            byte: 0xC5},
                KeyValue {desc: "KEY_F5".into(),            byte: 0xC6},
                KeyValue {desc: "KEY_F6".into(),            byte: 0xC7},
                KeyValue {desc: "KEY_F7".into(),            byte: 0xC8},
                KeyValue {desc: "KEY_F8".into(),            byte: 0xC9},
                KeyValue {desc: "KEY_F9".into(),            byte: 0xCA},
                KeyValue {desc: "KEY_F10".into(),           byte: 0xCB},
                KeyValue {desc: "KEY_F11".into(),           byte: 0xCC},
                KeyValue {desc: "KEY_F12".into(),           byte: 0xCD},
                KeyValue {desc: "KEY_F13".into(),           byte: 0xF0},
                KeyValue {desc: "KEY_F14".into(),           byte: 0xF1},
                KeyValue {desc: "KEY_F15".into(),           byte: 0xF2},
                KeyValue {desc: "KEY_F16".into(),           byte: 0xF3},
                KeyValue {desc: "KEY_F17".into(),           byte: 0xF4},
                KeyValue {desc: "KEY_F18".into(),           byte: 0xF5},
                KeyValue {desc: "KEY_F19".into(),           byte: 0xF6},
                KeyValue {desc: "KEY_F20".into(),           byte: 0xF7},
                KeyValue {desc: "KEY_F21".into(),           byte: 0xF8},
                KeyValue {desc: "KEY_F22".into(),           byte: 0xF9},
                KeyValue {desc: "KEY_F23".into(),           byte: 0xFA},
                KeyValue {desc: "KEY_F24".into(),           byte: 0xFB},

                //keys
                KeyValue {desc: "KEY_A".into(),                 byte: b'a'},
                KeyValue {desc: "KEY_B".into(),                 byte: b'b'},
                KeyValue {desc: "KEY_C".into(),                 byte: b'c'},
                KeyValue {desc: "KEY_D".into(),                 byte: b'd'},
                KeyValue {desc: "KEY_E".into(),                 byte: b'e'},
                KeyValue {desc: "KEY_F".into(),                 byte: b'f'},
                KeyValue {desc: "KEY_G".into(),                 byte: b'g'},
                KeyValue {desc: "KEY_H".into(),                 byte: b'h'},
                KeyValue {desc: "KEY_I".into(),                 byte: b'i'},
                KeyValue {desc: "KEY_J".into(),                 byte: b'j'},
                KeyValue {desc: "KEY_K".into(),                 byte: b'k'},
                KeyValue {desc: "KEY_L".into(),                 byte: b'l'},
                KeyValue {desc: "KEY_M".into(),                 byte: b'm'},
                KeyValue {desc: "KEY_N".into(),                 byte: b'n'},
                KeyValue {desc: "KEY_O".into(),                 byte: b'o'},
                KeyValue {desc: "KEY_P".into(),                 byte: b'p'},
                KeyValue {desc: "KEY_Q".into(),                 byte: b'q'},
                KeyValue {desc: "KEY_R".into(),                 byte: b'r'},
                KeyValue {desc: "KEY_S".into(),                 byte: b's'},
                KeyValue {desc: "KEY_T".into(),                 byte: b't'},
                KeyValue {desc: "KEY_U".into(),                 byte: b'u'},
                KeyValue {desc: "KEY_V".into(),                 byte: b'v'},
                KeyValue {desc: "KEY_W".into(),                 byte: b'w'},
                KeyValue {desc: "KEY_X".into(),                 byte: b'x'},
                KeyValue {desc: "KEY_Y".into(),                 byte: b'y'},
                KeyValue {desc: "KEY_Z".into(),                 byte: b'z'},
                KeyValue {desc: "KEY_1".into(),                 byte: b'1'},
                KeyValue {desc: "KEY_2".into(),                 byte: b'2'},
                KeyValue {desc: "KEY_3".into(),                 byte: b'3'},
                KeyValue {desc: "KEY_4".into(),                 byte: b'4'},
                KeyValue {desc: "KEY_5".into(),                 byte: b'5'},
                KeyValue {desc: "KEY_6".into(),                 byte: b'6'},
                KeyValue {desc: "KEY_7".into(),                 byte: b'7'},
                KeyValue {desc: "KEY_8".into(),                 byte: b'8'},
                KeyValue {desc: "KEY_9".into(),                 byte: b'9'},
                KeyValue {desc: "KEY_0".into(),                 byte: b'0'},
                KeyValue {desc: "KEY_?".into(),                 byte: b'?'},
            ]
        }
    }

    pub fn byte_key_index(&mut self, key: u8) -> usize {
        self.keys.iter().position(|k| k.byte == key).unwrap_or(0)
    }

    pub fn byte_key_by_desc(&mut self, key: SharedString) -> u8 {
        self.keys.iter().find(|&k| k.desc == key).unwrap_or(self.keys.get(0).unwrap()).byte
    }

    pub fn desc_key_by_byte(&mut self, key: u8) -> &SharedString {
        &self.keys.iter().find(|&k| k.byte == key).unwrap_or(self.keys.get(0).unwrap()).desc
    }
    
}