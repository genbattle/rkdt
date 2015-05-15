#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl super::Point for Vec3 {
    type V = f32;
    
    fn size(&self) -> usize {
        3
    }
    
    fn at(&self, pos: usize) -> f32 {
        match pos {
            1 => self.x,
            2 => self.y,
            3 => self.z,
            _ => 0.0
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

use std::fmt::{Debug, Formatter, Error};
impl Debug for Vec3 {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(fmt.write_fmt(format_args!("({:?},{:?},{:?})", self.x, self.y, self.z)));
        return Ok(());
    }
}