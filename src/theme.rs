#[allow(dead_code)]
pub mod everforest {
    pub const FOREGROUND: u32 = 0xD3C6AA;
    pub const RED: u32 = 0xE67E80;
    pub const YELLOW: u32 = 0xDBBC7F;
    pub const GREEN: u32 = 0xA7C080;
    pub const BLUE: u32 = 0x7FBBB3;
    pub const PURPLE: u32 = 0xD699B6;
    pub const AQUA: u32 = 0x83C092;
    pub const ORANGE: u32 = 0xE69875;
    pub const STATUSLINE_1: u32 = 0xA7C080;
    pub const STATUSLINE_2: u32 = 0xD3C6AA;
    pub const STATUSLINE_3: u32 = 0xE67E80;
    pub const GREY_0: u32 = 0x7A8478;
    pub const GREY_1: u32 = 0x859289;
    pub const GREY_2: u32 = 0x9DA9A0;
    pub const BACKGROUND_DIM: u32 = 0x232A2E;
    pub const BACKGROUND_0: u32 = 0x2D353B;
    pub const BACKGROUND_1: u32 = 0x343F44;
    pub const BACKGROUND_2: u32 = 0x3D484D;
    pub const BACKGROUND_3: u32 = 0x475258;
    pub const BACKGROUND_4: u32 = 0x4F585E;
    pub const BACKGROUND_5: u32 = 0x56635F;
    pub const BACKGROUND_RED: u32 = 0x514045;
    pub const BACKGROUND_YELLOW: u32 = 0x4D4C43;
    pub const BACKGROUND_GREEN: u32 = 0x425047;
    pub const BACKGROUND_BLUE: u32 = 0x3A515D;
    pub const BACKGROUND_PURPLE: u32 = 0x4A444E;
    pub const BACKGROUND_VISUAL: u32 = 0x543A48;
}

pub const BACKGROUND: u32 = everforest::BACKGROUND_0;
pub const TEXT: u32 = everforest::FOREGROUND;

pub const TITLEBAR_HEIGHT: f32 = 28.;
pub const TITLEBAR_BACKGROUND: u32 = everforest::BACKGROUND_0;
pub const TITLEBAR_BORDER_COLOR: u32 = everforest::BACKGROUND_DIM;
