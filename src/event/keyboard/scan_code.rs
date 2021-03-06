use crate::bind;

pub enum ScanCode {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Return,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    NonUsHash,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Period,
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadPeriod,
    NonUsBackslash,
    Application,
    Power,
    KeypadEquals,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    KeypadComma,
    KeypadEqualsAs400,
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AltErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    Keypad00,
    Keypad000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubunit,
    KeypadLeftParen,
    KeypadRightParen,
    KeypadLeftBrace,
    KeypadRightBrace,
    KeypadTab,
    KeypadBackspace,
    KeypadA,
    KeypadB,
    KeypadC,
    KeypadD,
    KeypadE,
    KeypadF,
    KeypadXor,
    KeypadPower,
    KeypadPercent,
    KeypadLess,
    KeypadGreater,
    KeypadAmpersand,
    KeypadDblAmpersand,
    KeypadVerticalBar,
    KeypadDblVerticalbar,
    KeypadColon,
    KeypadHash,
    KeypadSpace,
    KeypadAt,
    KeypadExclam,
    KeypadMemStore,
    KeypadMemRecall,
    KeypadMemClear,
    KeypadMemAdd,
    KeypadMemSubtract,
    KeypadMemMultiply,
    KeypadMemDivide,
    KeypadPlusMinus,
    KeypadClear,
    KeypadClearEntry,
    KeypadBinary,
    KeypadOctal,
    KeypadDecimal,
    KeypadHexadecimal,
    LCtrl,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    ApplicationSearch,
    ApplicationHome,
    ApplicationBack,
    ApplicationForward,
    ApplicationStop,
    ApplicationRefresh,
    ApplicationBookmarks,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KeyboardIllumToggle,
    KeyboardIllumDown,
    KeyboardIllumUp,
    Eject,
    Sleep,
    App1,
    App2,
    AudioRewind,
    AudioFastForward,
}

pub const NUM_SCANCODES: u16 = 512;

impl From<bind::SDL_Scancode> for ScanCode {
    fn from(code: bind::SDL_Scancode) -> Self {
        use ScanCode::*;
        match code {
            bind::SDL_Scancode_SDL_SCANCODE_A => A,
            bind::SDL_Scancode_SDL_SCANCODE_B => B,
            bind::SDL_Scancode_SDL_SCANCODE_C => C,
            bind::SDL_Scancode_SDL_SCANCODE_D => D,
            bind::SDL_Scancode_SDL_SCANCODE_E => E,
            bind::SDL_Scancode_SDL_SCANCODE_F => F,
            bind::SDL_Scancode_SDL_SCANCODE_G => G,
            bind::SDL_Scancode_SDL_SCANCODE_H => H,
            bind::SDL_Scancode_SDL_SCANCODE_I => I,
            bind::SDL_Scancode_SDL_SCANCODE_J => J,
            bind::SDL_Scancode_SDL_SCANCODE_K => K,
            bind::SDL_Scancode_SDL_SCANCODE_L => L,
            bind::SDL_Scancode_SDL_SCANCODE_M => M,
            bind::SDL_Scancode_SDL_SCANCODE_N => N,
            bind::SDL_Scancode_SDL_SCANCODE_O => O,
            bind::SDL_Scancode_SDL_SCANCODE_P => P,
            bind::SDL_Scancode_SDL_SCANCODE_Q => Q,
            bind::SDL_Scancode_SDL_SCANCODE_R => R,
            bind::SDL_Scancode_SDL_SCANCODE_S => S,
            bind::SDL_Scancode_SDL_SCANCODE_T => T,
            bind::SDL_Scancode_SDL_SCANCODE_U => U,
            bind::SDL_Scancode_SDL_SCANCODE_V => V,
            bind::SDL_Scancode_SDL_SCANCODE_W => W,
            bind::SDL_Scancode_SDL_SCANCODE_X => X,
            bind::SDL_Scancode_SDL_SCANCODE_Y => Y,
            bind::SDL_Scancode_SDL_SCANCODE_Z => Z,
            bind::SDL_Scancode_SDL_SCANCODE_1 => Num1,
            bind::SDL_Scancode_SDL_SCANCODE_2 => Num2,
            bind::SDL_Scancode_SDL_SCANCODE_3 => Num3,
            bind::SDL_Scancode_SDL_SCANCODE_4 => Num4,
            bind::SDL_Scancode_SDL_SCANCODE_5 => Num5,
            bind::SDL_Scancode_SDL_SCANCODE_6 => Num6,
            bind::SDL_Scancode_SDL_SCANCODE_7 => Num7,
            bind::SDL_Scancode_SDL_SCANCODE_8 => Num8,
            bind::SDL_Scancode_SDL_SCANCODE_9 => Num9,
            bind::SDL_Scancode_SDL_SCANCODE_0 => Num0,
            bind::SDL_Scancode_SDL_SCANCODE_RETURN => Return,
            bind::SDL_Scancode_SDL_SCANCODE_ESCAPE => Escape,
            bind::SDL_Scancode_SDL_SCANCODE_BACKSPACE => Backspace,
            bind::SDL_Scancode_SDL_SCANCODE_TAB => Tab,
            bind::SDL_Scancode_SDL_SCANCODE_SPACE => Space,
            bind::SDL_Scancode_SDL_SCANCODE_MINUS => Minus,
            bind::SDL_Scancode_SDL_SCANCODE_EQUALS => Equals,
            bind::SDL_Scancode_SDL_SCANCODE_LEFTBRACKET => LeftBracket,
            bind::SDL_Scancode_SDL_SCANCODE_RIGHTBRACKET => RightBracket,
            bind::SDL_Scancode_SDL_SCANCODE_BACKSLASH => Backslash,
            bind::SDL_Scancode_SDL_SCANCODE_NONUSHASH => NonUsHash,
            bind::SDL_Scancode_SDL_SCANCODE_SEMICOLON => Semicolon,
            bind::SDL_Scancode_SDL_SCANCODE_APOSTROPHE => Apostrophe,
            bind::SDL_Scancode_SDL_SCANCODE_GRAVE => Grave,
            bind::SDL_Scancode_SDL_SCANCODE_COMMA => Comma,
            bind::SDL_Scancode_SDL_SCANCODE_PERIOD => Period,
            bind::SDL_Scancode_SDL_SCANCODE_SLASH => Slash,
            bind::SDL_Scancode_SDL_SCANCODE_CAPSLOCK => CapsLock,
            bind::SDL_Scancode_SDL_SCANCODE_F1 => F1,
            bind::SDL_Scancode_SDL_SCANCODE_F2 => F2,
            bind::SDL_Scancode_SDL_SCANCODE_F3 => F3,
            bind::SDL_Scancode_SDL_SCANCODE_F4 => F4,
            bind::SDL_Scancode_SDL_SCANCODE_F5 => F5,
            bind::SDL_Scancode_SDL_SCANCODE_F6 => F6,
            bind::SDL_Scancode_SDL_SCANCODE_F7 => F7,
            bind::SDL_Scancode_SDL_SCANCODE_F8 => F8,
            bind::SDL_Scancode_SDL_SCANCODE_F9 => F9,
            bind::SDL_Scancode_SDL_SCANCODE_F10 => F10,
            bind::SDL_Scancode_SDL_SCANCODE_F11 => F11,
            bind::SDL_Scancode_SDL_SCANCODE_F12 => F12,
            bind::SDL_Scancode_SDL_SCANCODE_PRINTSCREEN => PrintScreen,
            bind::SDL_Scancode_SDL_SCANCODE_SCROLLLOCK => ScrollLock,
            bind::SDL_Scancode_SDL_SCANCODE_PAUSE => Pause,
            bind::SDL_Scancode_SDL_SCANCODE_INSERT => Insert,
            bind::SDL_Scancode_SDL_SCANCODE_HOME => Home,
            bind::SDL_Scancode_SDL_SCANCODE_PAGEUP => PageUp,
            bind::SDL_Scancode_SDL_SCANCODE_DELETE => Delete,
            bind::SDL_Scancode_SDL_SCANCODE_END => End,
            bind::SDL_Scancode_SDL_SCANCODE_PAGEDOWN => PageDown,
            bind::SDL_Scancode_SDL_SCANCODE_RIGHT => Right,
            bind::SDL_Scancode_SDL_SCANCODE_LEFT => Left,
            bind::SDL_Scancode_SDL_SCANCODE_DOWN => Down,
            bind::SDL_Scancode_SDL_SCANCODE_UP => Up,
            bind::SDL_Scancode_SDL_SCANCODE_NUMLOCKCLEAR => NumLockClear,
            bind::SDL_Scancode_SDL_SCANCODE_KP_DIVIDE => KeypadDivide,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MULTIPLY => KeypadMultiply,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MINUS => KeypadMinus,
            bind::SDL_Scancode_SDL_SCANCODE_KP_PLUS => KeypadPlus,
            bind::SDL_Scancode_SDL_SCANCODE_KP_ENTER => KeypadEnter,
            bind::SDL_Scancode_SDL_SCANCODE_KP_1 => Keypad1,
            bind::SDL_Scancode_SDL_SCANCODE_KP_2 => Keypad2,
            bind::SDL_Scancode_SDL_SCANCODE_KP_3 => Keypad3,
            bind::SDL_Scancode_SDL_SCANCODE_KP_4 => Keypad4,
            bind::SDL_Scancode_SDL_SCANCODE_KP_5 => Keypad5,
            bind::SDL_Scancode_SDL_SCANCODE_KP_6 => Keypad6,
            bind::SDL_Scancode_SDL_SCANCODE_KP_7 => Keypad7,
            bind::SDL_Scancode_SDL_SCANCODE_KP_8 => Keypad8,
            bind::SDL_Scancode_SDL_SCANCODE_KP_9 => Keypad9,
            bind::SDL_Scancode_SDL_SCANCODE_KP_0 => Keypad0,
            bind::SDL_Scancode_SDL_SCANCODE_KP_PERIOD => KeypadPeriod,
            bind::SDL_Scancode_SDL_SCANCODE_NONUSBACKSLASH => NonUsBackslash,
            bind::SDL_Scancode_SDL_SCANCODE_APPLICATION => Application,
            bind::SDL_Scancode_SDL_SCANCODE_POWER => Power,
            bind::SDL_Scancode_SDL_SCANCODE_KP_EQUALS => KeypadEquals,
            bind::SDL_Scancode_SDL_SCANCODE_F13 => F13,
            bind::SDL_Scancode_SDL_SCANCODE_F14 => F14,
            bind::SDL_Scancode_SDL_SCANCODE_F15 => F15,
            bind::SDL_Scancode_SDL_SCANCODE_F16 => F16,
            bind::SDL_Scancode_SDL_SCANCODE_F17 => F17,
            bind::SDL_Scancode_SDL_SCANCODE_F18 => F18,
            bind::SDL_Scancode_SDL_SCANCODE_F19 => F19,
            bind::SDL_Scancode_SDL_SCANCODE_F20 => F20,
            bind::SDL_Scancode_SDL_SCANCODE_F21 => F21,
            bind::SDL_Scancode_SDL_SCANCODE_F22 => F22,
            bind::SDL_Scancode_SDL_SCANCODE_F23 => F23,
            bind::SDL_Scancode_SDL_SCANCODE_F24 => F24,
            bind::SDL_Scancode_SDL_SCANCODE_EXECUTE => Execute,
            bind::SDL_Scancode_SDL_SCANCODE_HELP => Help,
            bind::SDL_Scancode_SDL_SCANCODE_MENU => Menu,
            bind::SDL_Scancode_SDL_SCANCODE_SELECT => Select,
            bind::SDL_Scancode_SDL_SCANCODE_STOP => Stop,
            bind::SDL_Scancode_SDL_SCANCODE_AGAIN => Again,
            bind::SDL_Scancode_SDL_SCANCODE_UNDO => Undo,
            bind::SDL_Scancode_SDL_SCANCODE_CUT => Cut,
            bind::SDL_Scancode_SDL_SCANCODE_COPY => Copy,
            bind::SDL_Scancode_SDL_SCANCODE_PASTE => Paste,
            bind::SDL_Scancode_SDL_SCANCODE_FIND => Find,
            bind::SDL_Scancode_SDL_SCANCODE_MUTE => Mute,
            bind::SDL_Scancode_SDL_SCANCODE_VOLUMEUP => VolumeUp,
            bind::SDL_Scancode_SDL_SCANCODE_VOLUMEDOWN => VolumeDown,
            bind::SDL_Scancode_SDL_SCANCODE_KP_COMMA => KeypadComma,
            bind::SDL_Scancode_SDL_SCANCODE_KP_EQUALSAS400 => KeypadEqualsAs400,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL1 => International1,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL2 => International2,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL3 => International3,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL4 => International4,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL5 => International5,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL6 => International6,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL7 => International7,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL8 => International8,
            bind::SDL_Scancode_SDL_SCANCODE_INTERNATIONAL9 => International9,
            bind::SDL_Scancode_SDL_SCANCODE_LANG1 => Lang1,
            bind::SDL_Scancode_SDL_SCANCODE_LANG2 => Lang2,
            bind::SDL_Scancode_SDL_SCANCODE_LANG3 => Lang3,
            bind::SDL_Scancode_SDL_SCANCODE_LANG4 => Lang4,
            bind::SDL_Scancode_SDL_SCANCODE_LANG5 => Lang5,
            bind::SDL_Scancode_SDL_SCANCODE_LANG6 => Lang6,
            bind::SDL_Scancode_SDL_SCANCODE_LANG7 => Lang7,
            bind::SDL_Scancode_SDL_SCANCODE_LANG8 => Lang8,
            bind::SDL_Scancode_SDL_SCANCODE_LANG9 => Lang9,
            bind::SDL_Scancode_SDL_SCANCODE_ALTERASE => AltErase,
            bind::SDL_Scancode_SDL_SCANCODE_SYSREQ => SysReq,
            bind::SDL_Scancode_SDL_SCANCODE_CANCEL => Cancel,
            bind::SDL_Scancode_SDL_SCANCODE_CLEAR => Clear,
            bind::SDL_Scancode_SDL_SCANCODE_PRIOR => Prior,
            bind::SDL_Scancode_SDL_SCANCODE_RETURN2 => Return2,
            bind::SDL_Scancode_SDL_SCANCODE_SEPARATOR => Separator,
            bind::SDL_Scancode_SDL_SCANCODE_OUT => Out,
            bind::SDL_Scancode_SDL_SCANCODE_OPER => Oper,
            bind::SDL_Scancode_SDL_SCANCODE_CLEARAGAIN => ClearAgain,
            bind::SDL_Scancode_SDL_SCANCODE_CRSEL => CrSel,
            bind::SDL_Scancode_SDL_SCANCODE_EXSEL => ExSel,
            bind::SDL_Scancode_SDL_SCANCODE_KP_00 => Keypad00,
            bind::SDL_Scancode_SDL_SCANCODE_KP_000 => Keypad000,
            bind::SDL_Scancode_SDL_SCANCODE_THOUSANDSSEPARATOR => ThousandsSeparator,
            bind::SDL_Scancode_SDL_SCANCODE_DECIMALSEPARATOR => DecimalSeparator,
            bind::SDL_Scancode_SDL_SCANCODE_CURRENCYUNIT => CurrencyUnit,
            bind::SDL_Scancode_SDL_SCANCODE_CURRENCYSUBUNIT => CurrencySubunit,
            bind::SDL_Scancode_SDL_SCANCODE_KP_LEFTPAREN => KeypadLeftParen,
            bind::SDL_Scancode_SDL_SCANCODE_KP_RIGHTPAREN => KeypadRightParen,
            bind::SDL_Scancode_SDL_SCANCODE_KP_LEFTBRACE => KeypadLeftBrace,
            bind::SDL_Scancode_SDL_SCANCODE_KP_RIGHTBRACE => KeypadRightBrace,
            bind::SDL_Scancode_SDL_SCANCODE_KP_TAB => KeypadTab,
            bind::SDL_Scancode_SDL_SCANCODE_KP_BACKSPACE => KeypadBackspace,
            bind::SDL_Scancode_SDL_SCANCODE_KP_A => KeypadA,
            bind::SDL_Scancode_SDL_SCANCODE_KP_B => KeypadB,
            bind::SDL_Scancode_SDL_SCANCODE_KP_C => KeypadC,
            bind::SDL_Scancode_SDL_SCANCODE_KP_D => KeypadD,
            bind::SDL_Scancode_SDL_SCANCODE_KP_E => KeypadE,
            bind::SDL_Scancode_SDL_SCANCODE_KP_F => KeypadF,
            bind::SDL_Scancode_SDL_SCANCODE_KP_XOR => KeypadXor,
            bind::SDL_Scancode_SDL_SCANCODE_KP_POWER => KeypadPower,
            bind::SDL_Scancode_SDL_SCANCODE_KP_PERCENT => KeypadPercent,
            bind::SDL_Scancode_SDL_SCANCODE_KP_LESS => KeypadLess,
            bind::SDL_Scancode_SDL_SCANCODE_KP_GREATER => KeypadGreater,
            bind::SDL_Scancode_SDL_SCANCODE_KP_AMPERSAND => KeypadAmpersand,
            bind::SDL_Scancode_SDL_SCANCODE_KP_DBLAMPERSAND => KeypadDblAmpersand,
            bind::SDL_Scancode_SDL_SCANCODE_KP_VERTICALBAR => KeypadVerticalBar,
            bind::SDL_Scancode_SDL_SCANCODE_KP_DBLVERTICALBAR => KeypadDblVerticalbar,
            bind::SDL_Scancode_SDL_SCANCODE_KP_COLON => KeypadColon,
            bind::SDL_Scancode_SDL_SCANCODE_KP_HASH => KeypadHash,
            bind::SDL_Scancode_SDL_SCANCODE_KP_SPACE => KeypadSpace,
            bind::SDL_Scancode_SDL_SCANCODE_KP_AT => KeypadAt,
            bind::SDL_Scancode_SDL_SCANCODE_KP_EXCLAM => KeypadExclam,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMSTORE => KeypadMemStore,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMRECALL => KeypadMemRecall,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMCLEAR => KeypadMemClear,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMADD => KeypadMemAdd,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMSUBTRACT => KeypadMemSubtract,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMMULTIPLY => KeypadMemMultiply,
            bind::SDL_Scancode_SDL_SCANCODE_KP_MEMDIVIDE => KeypadMemDivide,
            bind::SDL_Scancode_SDL_SCANCODE_KP_PLUSMINUS => KeypadPlusMinus,
            bind::SDL_Scancode_SDL_SCANCODE_KP_CLEAR => KeypadClear,
            bind::SDL_Scancode_SDL_SCANCODE_KP_CLEARENTRY => KeypadClearEntry,
            bind::SDL_Scancode_SDL_SCANCODE_KP_BINARY => KeypadBinary,
            bind::SDL_Scancode_SDL_SCANCODE_KP_OCTAL => KeypadOctal,
            bind::SDL_Scancode_SDL_SCANCODE_KP_DECIMAL => KeypadDecimal,
            bind::SDL_Scancode_SDL_SCANCODE_KP_HEXADECIMAL => KeypadHexadecimal,
            bind::SDL_Scancode_SDL_SCANCODE_LCTRL => LCtrl,
            bind::SDL_Scancode_SDL_SCANCODE_LSHIFT => LShift,
            bind::SDL_Scancode_SDL_SCANCODE_LALT => LAlt,
            bind::SDL_Scancode_SDL_SCANCODE_LGUI => LGui,
            bind::SDL_Scancode_SDL_SCANCODE_RCTRL => RCtrl,
            bind::SDL_Scancode_SDL_SCANCODE_RSHIFT => RShift,
            bind::SDL_Scancode_SDL_SCANCODE_RALT => RAlt,
            bind::SDL_Scancode_SDL_SCANCODE_RGUI => RGui,
            bind::SDL_Scancode_SDL_SCANCODE_MODE => Mode,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIONEXT => AudioNext,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOPREV => AudioPrev,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOSTOP => AudioStop,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOPLAY => AudioPlay,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOMUTE => AudioMute,
            bind::SDL_Scancode_SDL_SCANCODE_MEDIASELECT => MediaSelect,
            bind::SDL_Scancode_SDL_SCANCODE_WWW => Www,
            bind::SDL_Scancode_SDL_SCANCODE_MAIL => Mail,
            bind::SDL_Scancode_SDL_SCANCODE_CALCULATOR => Calculator,
            bind::SDL_Scancode_SDL_SCANCODE_COMPUTER => Computer,
            bind::SDL_Scancode_SDL_SCANCODE_AC_SEARCH => ApplicationSearch,
            bind::SDL_Scancode_SDL_SCANCODE_AC_HOME => ApplicationHome,
            bind::SDL_Scancode_SDL_SCANCODE_AC_BACK => ApplicationBack,
            bind::SDL_Scancode_SDL_SCANCODE_AC_FORWARD => ApplicationForward,
            bind::SDL_Scancode_SDL_SCANCODE_AC_STOP => ApplicationStop,
            bind::SDL_Scancode_SDL_SCANCODE_AC_REFRESH => ApplicationRefresh,
            bind::SDL_Scancode_SDL_SCANCODE_AC_BOOKMARKS => ApplicationBookmarks,
            bind::SDL_Scancode_SDL_SCANCODE_BRIGHTNESSDOWN => BrightnessDown,
            bind::SDL_Scancode_SDL_SCANCODE_BRIGHTNESSUP => BrightnessUp,
            bind::SDL_Scancode_SDL_SCANCODE_DISPLAYSWITCH => DisplaySwitch,
            bind::SDL_Scancode_SDL_SCANCODE_KBDILLUMTOGGLE => KeyboardIllumToggle,
            bind::SDL_Scancode_SDL_SCANCODE_KBDILLUMDOWN => KeyboardIllumDown,
            bind::SDL_Scancode_SDL_SCANCODE_KBDILLUMUP => KeyboardIllumUp,
            bind::SDL_Scancode_SDL_SCANCODE_EJECT => Eject,
            bind::SDL_Scancode_SDL_SCANCODE_SLEEP => Sleep,
            bind::SDL_Scancode_SDL_SCANCODE_APP1 => App1,
            bind::SDL_Scancode_SDL_SCANCODE_APP2 => App2,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOREWIND => AudioRewind,
            bind::SDL_Scancode_SDL_SCANCODE_AUDIOFASTFORWARD => AudioFastForward,
            _ => Unknown,
        }
    }
}