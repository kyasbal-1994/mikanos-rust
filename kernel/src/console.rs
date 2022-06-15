use core::borrow::Borrow;
use crate::{Renderable, screen};

pub struct ConsoleWrite;

pub struct Console{
    size: (u32,u32),
    cursor: (u32,u32),
    color: [u8;3]
}

impl Console{
    pub fn new()->Self{
        let a = ConsoleWrite;
        let b = a.borrow();
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