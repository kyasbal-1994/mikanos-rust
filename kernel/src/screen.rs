use core::slice;
use shared::framebuffer;
use shared::framebuffer::{FrameBuffer, PixelFormat};

static ASCII_FONT: &[u8; 4096] = include_bytes!("../ascii.bin");
// Cursor design cited from https://codegolf.stackexchange.com/questions/211641/cat-a-mouse-ascii-art-pointers
static MOUSE_CURSOR: &str =
    concat!(r"@       "
    , r"@@      "
    , r"@@@     "
    , r"@@@@    "
    , r"@@@@@   "
    , r"@@@@@@  "
    , r"@@@@@@@ "
    , r"@@@@    "
    , r"@@ @@   "
    , r"@  @@   "
    , r"    @@  "
    , r"    @@  "
    , r"     @@ "
    , r"     @@ "
    , r"      @@"
    , r"      @@");
static MOUSE_CURSOR_RESOLUTION: (u32, u32) = (8, 16);

pub static mut MAIN_SCREEN: Option<Screen> = Option::None;

pub fn initialize_screen(fb: &framebuffer::FrameBuffer) {
    unsafe {
        MAIN_SCREEN = Some((*fb).into())
    }
}

/// Represents the screen buffer directly shown to users
/// This will be initialized with a framebuffer obtained from UEFI
pub struct Screen {
    target: framebuffer::FrameBuffer,
}

impl Screen {
    pub fn new(fb: framebuffer::FrameBuffer) -> Self {
        Screen {
            target: fb
        }
    }
}

pub trait Renderable {
    fn bytes(&self) -> &[u8];

    fn bytes_mut(&mut self) -> &mut [u8];

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn stride(&self) -> usize;

    fn format(&self) -> PixelFormat;

    fn put_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        if x < self.width() && y < self.height() {
            let format = self.format();
            let b = self.to_offset(x, y);
            let mut fb = self.bytes_mut();
            if format == PixelFormat::Rgb {
                fb[b + 0] = color[0];
                fb[b + 1] = color[1];
                fb[b + 2] = color[2];
            } else {
                fb[b + 0] = color[2];
                fb[b + 1] = color[1];
                fb[b + 2] = color[0];
            }
        }
    }

    fn draw_char(&mut self, x: u32, y: u32, c: char, color: [u8; 3]) {
        let ci = (c as u8) as usize;
        let font_begin = ci * 16;
        let font = &ASCII_FONT[font_begin..(font_begin + 16 * 8)];
        for xp in 0..8 {
            for yp in 0..16 {
                if font[yp] & (1 << (7 - xp)) > 0 {
                    self.put_pixel(x + xp, y + (yp as u32), color);
                }
            }
        }
    }

    fn draw_string(&mut self, x: u32, y: u32, s: &str, color: [u8; 3]) {
        let mut offset = 0;
        for c in s.chars() {
            self.draw_char(x + offset, y, c, color);
            offset += 8;
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: [u8; 3]) {
        for xp in x..(x + w) {
            for yp in y..(y + h) {
                self.put_pixel(xp, yp, color);
            }
        }
    }

    fn draw_aa(&mut self, x: u32, y: u32, aa: &str, aa_resolution: (u32, u32), color: [u8; 3]) {
        let (w, h) = aa_resolution;
        let mut chars = aa.chars();
        for yp in y..(y + h) {
            for xp in x..(x + w) {
                let c = chars.next().unwrap();
                if c == '@' {
                    self.put_pixel(xp, yp, color);
                }
            }
        }
    }

    fn draw_cursor(&mut self, x: u32, y: u32, color: [u8; 3]) {
        self.draw_aa(x, y, MOUSE_CURSOR, MOUSE_CURSOR_RESOLUTION, color);
    }


    fn clear(&mut self, color: [u8; 3]) {
        self.draw_rect(0, 0, self.width(), self.height(), color);
    }

    fn to_offset(&self, x: u32, y: u32) -> usize {
        ((self.width() * y + x) * 4) as usize
    }
}

impl Renderable for Screen {
    fn bytes(&self) -> &[u8] {
        unsafe {
            let (_, resY) = self.target.resolution;
            slice::from_raw_parts(self.target.framebuffer, (self.target.stride * resY * 4) as usize)
        }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            let (_, resY) = self.target.resolution;
            slice::from_raw_parts_mut(self.target.framebuffer, (self.target.stride * resY * 4) as usize)
        }
    }

    fn stride(&self) -> usize {
        self.target.stride as usize
    }

    fn format(&self) -> PixelFormat {
        self.target.format
    }

    fn width(&self) -> u32 {
        return self.target.resolution.0;
    }

    fn height(&self) -> u32 {
        return self.target.resolution.1;
    }

    fn to_offset(&self, x: u32, y: u32) -> usize {
        ((self.target.stride * y + x) * 4) as usize
    }
}

impl From<FrameBuffer> for Screen {
    fn from(fb: FrameBuffer) -> Self {
        Self {
            target: fb
        }
    }
}