#[repr(C)]
#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Copy, Clone)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone,Copy)]
pub struct FrameBuffer{
    pub framebuffer: *mut u8,
    pub stride: u32,
    pub resolution: (u32,u32),
    pub format: PixelFormat,
}