use core::borrow::Borrow;
use core::fmt;
use crate::{Renderable, screen};

pub static mut MAIN_CONSOLE: Option<Console> = Option::None;

pub fn initialize_console(){
    unsafe {
        MAIN_CONSOLE = Some(Console::new());
    }
}

pub struct Console{
    size: (u32,u32),
    cursor: (u32,u32),
    color: [u8;3]
}

impl Console{
    pub fn new()->Self{
        Self {
            size: (80,25),
            cursor: (0,0),
            color: [0,0,0]
        }
    }

    fn put_char_direct(&self,x:u32,y:u32,c: char){
        unsafe{
            screen::MAIN_SCREEN.as_mut().unwrap().draw_char(
                x,y,c,self.color
            )
        }
    }

    pub fn put_str(&mut self,s: &str){
        for c in s.chars(){
            if c == '\n' {
                let (x,y) = self.cursor;
                self.cursor = (0,y+1);
            }else{
                if self.cursor.0 < self.size.0{
                    self.put_char_direct(self.cursor.0 * 8 , self.cursor.1 * 20,c);
                    let (x,y) = self.cursor;
                    self.cursor = (x+1,y);
                }else{
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ConsoleWrite;

impl fmt::Write for ConsoleWrite{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe{
            MAIN_CONSOLE.as_mut().unwrap().put_str(s);
        }
        Ok(())
    }
}

#[allow(unused_macros)]
macro_rules! println{
        ($( $t:tt )*) => {{
        use core::fmt::Write;
        writeln!(crate::console::ConsoleWrite, $( $t )*).unwrap();
    }};
}