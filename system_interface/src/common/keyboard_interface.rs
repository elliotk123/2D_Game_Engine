pub trait KeyboardBackend
{
    fn poll_events(&mut self) -> Vec<MyKeyboardEvent>;

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MyKeyboardEvent {
    KeyDown(MyKey),
    KeyUp(MyKey),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MyKey {
    A = 97,              // SDLK_a
    B = 98,              // SDLK_b
    C = 99,              // SDLK_c
    D = 100,             // SDLK_d
    E = 101,             // SDLK_e
    F = 102,             // SDLK_f
    G = 103,             // SDLK_g
    H = 104,             // SDLK_h
    I = 105,             // SDLK_i
    J = 106,             // SDLK_j
    K = 107,             // SDLK_k
    L = 108,             // SDLK_l
    M = 109,             // SDLK_m
    N = 110,             // SDLK_n
    O = 111,             // SDLK_o
    P = 112,             // SDLK_p
    Q = 113,             // SDLK_q
    R = 114,             // SDLK_r
    S = 115,             // SDLK_s
    T = 116,             // SDLK_t
    U = 117,             // SDLK_u
    V = 118,             // SDLK_v
    W = 119,             // SDLK_w
    X = 120,             // SDLK_x
    Y = 121,             // SDLK_y
    Z = 122,             // SDLK_z
    Num0 = 48,           // SDLK_0
    Num1 = 49,           // SDLK_1
    Num2 = 50,           // SDLK_2
    Num3 = 51,           // SDLK_3
    Num4 = 52,           // SDLK_4
    Num5 = 53,           // SDLK_5
    Num6 = 54,           // SDLK_6
    Num7 = 55,           // SDLK_7
    Num8 = 56,           // SDLK_8
    Num9 = 57,           // SDLK_9
    Escape = 27,         // SDLK_ESCAPE
    LeftControl = 1073742048,   // SDLK_LCTRL
    LeftShift = 1073742049,     // SDLK_LSHIFT
    LeftAlt = 1073742050,       // SDLK_LALT
    LeftSuper = 1073742051,     // SDLK_LGUI
    RightControl = 1073742052,  // SDLK_RCTRL
    RightShift = 1073742053,    // SDLK_RSHIFT
    RightAlt = 1073742054,      // SDLK_RALT
    RightSuper = 1073742055,    // SDLK_RGUI
    Menu = 1073742067,          // SDLK_MENU
    Space = 32,                 // SDLK_SPACE
    Enter = 13,                 // SDLK_RETURN
    Backspace = 8,              // SDLK_BACKSPACE
    Tab = 9,                    // SDLK_TAB
    CapsLock = 1073741881,      // SDLK_CAPSLOCK
    F1 = 1073741882,            // SDLK_F1
    F2 = 1073741883,            // SDLK_F2
    F3 = 1073741884,            // SDLK_F3
    F4 = 1073741885,            // SDLK_F4
    F5 = 1073741886,            // SDLK_F5
    F6 = 1073741887,            // SDLK_F6
    F7 = 1073741888,            // SDLK_F7
    F8 = 1073741889,            // SDLK_F8
    F9 = 1073741890,            // SDLK_F9
    F10 = 1073741891,           // SDLK_F10
    F11 = 1073741892,           // SDLK_F11
    F12 = 1073741893,           // SDLK_F12
}

use std::convert::TryFrom;

impl TryFrom<i32> for MyKey {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            97 => Ok(MyKey::A),
            98 => Ok(MyKey::B),
            99 => Ok(MyKey::C),
            100 => Ok(MyKey::D),
            101 => Ok(MyKey::E),
            102 => Ok(MyKey::F),
            103 => Ok(MyKey::G),
            104 => Ok(MyKey::H),
            105 => Ok(MyKey::I),
            106 => Ok(MyKey::J),
            107 => Ok(MyKey::K),
            108 => Ok(MyKey::L),
            109 => Ok(MyKey::M),
            110 => Ok(MyKey::N),
            111 => Ok(MyKey::O),
            112 => Ok(MyKey::P),
            113 => Ok(MyKey::Q),
            114 => Ok(MyKey::R),
            115 => Ok(MyKey::S),
            116 => Ok(MyKey::T),
            117 => Ok(MyKey::U),
            118 => Ok(MyKey::V),
            119 => Ok(MyKey::W),
            120 => Ok(MyKey::X),
            121 => Ok(MyKey::Y),
            122 => Ok(MyKey::Z),
            48 => Ok(MyKey::Num0),
            49 => Ok(MyKey::Num1),
            50 => Ok(MyKey::Num2),
            51 => Ok(MyKey::Num3),
            52 => Ok(MyKey::Num4),
            53 => Ok(MyKey::Num5),
            54 => Ok(MyKey::Num6),
            55 => Ok(MyKey::Num7),
            56 => Ok(MyKey::Num8),
            57 => Ok(MyKey::Num9),
            27 => Ok(MyKey::Escape),
            1073742048 => Ok(MyKey::LeftControl),
            1073742049 => Ok(MyKey::LeftShift),
            1073742050 => Ok(MyKey::LeftAlt),
            1073742051 => Ok(MyKey::LeftSuper),
            1073742052 => Ok(MyKey::RightControl),
            1073742053 => Ok(MyKey::RightShift),
            1073742054 => Ok(MyKey::RightAlt),
            1073742055 => Ok(MyKey::RightSuper),
            1073742067 => Ok(MyKey::Menu),
            32 => Ok(MyKey::Space),
            13 => Ok(MyKey::Enter),
            8 => Ok(MyKey::Backspace),
            9 => Ok(MyKey::Tab),
            1073741881 => Ok(MyKey::CapsLock),
            1073741882 => Ok(MyKey::F1),
            1073741883 => Ok(MyKey::F2),
            1073741884 => Ok(MyKey::F3),
            1073741885 => Ok(MyKey::F4),
            1073741886 => Ok(MyKey::F5),
            1073741887 => Ok(MyKey::F6),
            1073741888 => Ok(MyKey::F7),
            1073741889 => Ok(MyKey::F8),
            1073741890 => Ok(MyKey::F9),
            1073741891 => Ok(MyKey::F10),
            1073741892 => Ok(MyKey::F11),
            1073741893 => Ok(MyKey::F12),
            _ => Err(()),
        }
    }
}   


pub struct KeyboardInterface
{
    pub backends : Vec<Box<dyn KeyboardBackend>>,
}

impl KeyboardInterface
{
    pub fn new(backends : Vec<Box<dyn KeyboardBackend>>) -> KeyboardInterface
    {
        KeyboardInterface
        {
            backends
        }
    }

    pub fn poll_events(&mut self) -> Vec<MyKeyboardEvent>
    {
        let mut all_events: Vec<MyKeyboardEvent> = Vec::new();
        for backend in self.backends.iter_mut()
        {
            let mut events = backend.poll_events();
            all_events.append(&mut events);
        }
        all_events
    }
}