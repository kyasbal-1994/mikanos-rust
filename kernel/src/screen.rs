use core::slice;
use shared::framebuffer;
use shared::framebuffer::PixelFormat;

static ASCII_FONT: &[u8; 4096] = include_bytes!("../ascii.bin");

/// Represents the screen buffer directly shown to users
/// This will be initialized with a framebuffer obtained from UEFI
pub struct Screen<'fb_lifetime>{
    target: &'fb_lifetime framebuffer::FrameBuffer,
    framebuffer: &'fb_lifetime mut[u8],
}

impl<'a> Screen<'a>{

    pub fn new(fb: &'a framebuffer::FrameBuffer)->Self{
        let (_,resY) = fb.resolution;
        Screen{
            target:fb,
            framebuffer: unsafe {
                slice::from_raw_parts_mut((&fb).framebuffer,((&fb).stride*resY*4) as usize)
            }
        }
    }
}

pub trait Renderable {

    fn get_width(&self)->u32;

    fn get_height(&self)->u32;

    fn write_pixel(&mut self,x:u32,y:u32,color:[u8;3]);

    fn write_char(&mut self,x:u32,y:u32,c:char,color:[u8;3]){
        let ci = (c as u8) as usize;
        let font_begin = ci * 16;
        let font = &ASCII_FONT[font_begin..(font_begin+16*8)];
        for xp in 0..8 {
            for yp in 0..16{
                if font[yp] & (1<<(7-xp)) > 0 {
                    self.write_pixel(x+xp,y+(yp as u32),color);
                }
            }
        }
    }

    fn write_rect(&mut self,x:u32,y:u32,w:u32,h:u32,color:[u8; 3]){
        for xp in x..(x+w) {
            for yp in y..(y+h) {
                self.write_pixel(xp,yp,color);
            }
        }
    }

    fn clear(&mut self,color:[u8;3]){
        self.write_rect(0,0,self.get_width(),self.get_height(),color);
    }

    fn to_offset(&self,x:u32,y:u32)->usize{
        ((self.get_width()*y + x) * 4) as usize
    }
}

impl<'a> Renderable for Screen<'a>{
    fn get_width(&self) -> u32 {
        return self.target.resolution.0;
    }

    fn get_height(&self) -> u32 {
        return self.target.resolution.1;
    }

    fn write_pixel(&mut self,x: u32, y: u32, color: [u8; 3]) {
        let (resX,resY) = self.target.resolution;
        if x < resX && y < resY {
            let b = self.to_offset(x,y);
            if self.target.format == PixelFormat::Rgb{
                self.framebuffer[b + 0] = color[0];
                self.framebuffer[b + 1] = color[1];
                self.framebuffer[b + 2] = color[2];
            }else {
                self.framebuffer[b + 0] = color[2];
                self.framebuffer[b + 1] = color[1];
                self.framebuffer[b + 2] = color[0];
            }
        }
    }

    fn to_offset(&self,x:u32,y:u32)->usize{
        ((self.target.stride * y + x) * 4) as usize
    }

}