#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl super::Point for Vec2 {
    type V = f32;
    
    fn size(&self) -> usize {
        2
    }
    
    fn at(&self, pos: usize) -> f32 {
        match pos {
            1 => self.x,
            2 => self.y,
            _ => 0.0
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

use std::fmt::{Debug, Formatter, Error};
impl Debug for Vec2 {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(fmt.write_fmt(format_args!("({:?},{:?})", self.x, self.y)));
        return Ok(());
    }
}