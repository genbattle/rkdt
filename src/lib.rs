pub mod rkdt {
    pub trait Point: PartialEq + Clone + Debug {
        type V: PartialOrd + PartialEq + Copy + Debug;
        // Returns the number of dimensions, k
        fn size(&self) -> usize;
        // For pos = (0 to k-1) returns a value
        fn at(&self, pos: usize) -> Self::V;
    }
    
    // TODO: Move this out of lib.rs into a separate file
    #[derive(Clone, Copy)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32
    }
    
    impl Point for Vec2 {
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
    
    #[derive(Clone, Copy)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32
    }

    // TODO: Move this out of lib.rs into a separate file
    impl Point for Vec3 {
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
    
    impl Debug for Vec3 {
        fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
            try!(fmt.write_fmt(format_args!("({:?},{:?},{:?})", self.x, self.y, self.z)));
            return Ok(());
        }
    }

    // TODO: Try to eliminate Boxes by introducing more static parmeters as in from_slice()?
    pub struct Node<T: Point> {
        pub value: T,
        pub left: Option<Box<Node<T>>>,
        pub right: Option<Box<Node<T>>>
    }

    impl<T: Point> Node<T> {
        pub fn new(val: T, l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) -> Node<T> {
            Node{value: val, left: l, right: r}
        }
        
        pub fn value(&self) -> &T {
            &self.value
        }
        
        // insert a new value into the tree
        pub fn insert(&mut self, value: &T) {
            self.insert_node(value, 0, value.size());
        }
        
        // Internal helper for printing
        /*fn format_node(&self, fmt: &mut Formatter) -> Result<(), Error> {
            try!(fmt.write_str("["));
            try!(fmt.write_fmt(format_args!("{:?}", self.value)));
            match self.left {
                Some(ref n) => {
                    try!(fmt.write_str(", "));
                    try!(self.format_node(n, fmt));
                },
                None => ()
            }
            match self.right {
                Some(ref n) => {
                    try!(fmt.write_str(", "));
                    try!(self.format_node(n, fmt));
                },
                None => ()
            }
            try!(fmt.write_str("]"));
            return ();
        }*/
        
        fn insert_node(&mut self, value: &T, axis: usize, k: usize) {
            if value.at(axis) < self.value.at(axis) {
                match self.left {
                    Some(ref mut n) => n.insert_node(value, (axis + 1) % k, k),
                    None => self.left = Some(Box::new(Node{value: value.clone(), left: None, right: None}))
                }
            } else if value.at(axis) > self.value.at(axis) {
                match self.right {
                    Some(ref mut n) => n.insert_node(value, (axis + 1) % k, k),
                    None => self.right = Some(Box::new(Node{value: value.clone(), left: None, right: None}))
                }
            }
        }
        
        // TODO: implement range search
        // TODO: implement k-nearest-neighbour search
        // TODO: implement search and delete
        // TODO: add special root node type? (maybe using newtype pattern)
        // TODO: implement kd-tree-based kmeans
    }
    
    impl<T: Point> Debug for Node<T> {
        // Pretty print the k-d tree to the console
        fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
            try!(fmt.write_str("["));
            try!(fmt.write_fmt(format_args!("{:?}", self.value)));
            match self.left {
                Some(ref n) => {
                    try!(fmt.write_str(", "));
                    try!(n.fmt(fmt));
                },
                None => ()
            }
            match self.right {
                Some(ref n) => {
                    try!(fmt.write_str(", "));
                    try!(n.fmt(fmt));
                },
                None => ()
            }
            try!(fmt.write_str("]"));
            return Ok(());
        }
    }
    
    /*
    Build a k-d tree from a slice of points. Algorithm taken from:
    http://en.wikipedia.org/wiki/K-d_tree
    */
    pub fn from_slice<T: Point>(points: &mut [T]) -> Option<Box<Node<T>>> {
        create_nodes(points, 0)
    }
    
    fn create_nodes<T: Point>(points: &mut [T], depth: usize) -> Option<Box<Node<T>>> {
        if points.is_empty() {
            return None;
        }
        let k = points[0].size();
        let axis = depth % k;
        // This code will panic if the point values are floating point NaN or another incomparable value
        points.sort_by(|a, b| {
            a.at(axis)
            .partial_cmp(&b.at(axis))
            .unwrap()
        });
        let median_index = points.len() / 2;
        Some(Box::new(Node {
            value: points[median_index].clone(),
            left: create_nodes(&mut points[..median_index], depth + 1),
            right: create_nodes(&mut points[(median_index + 1)..], depth + 1)
        }))
    }
}

#[test]
fn test_construction() {
    use rkdt::Vec2;
    let mut v = vec!(Vec2{x: 2.0, y: 3.0}, Vec2{x: 5.0, y: 4.0}, Vec2{x: 9.0, y: 6.0}, Vec2{x: 4.0, y: 7.0}, Vec2{x: 8.0, y: 1.0}, Vec2{x: 7.0, y: 2.0});
    let kdt = rkdt::from_slice(&mut v[..]).unwrap();
    println!("{:?}", kdt);
    assert!(kdt.value == v[6]);
}

// TODO: Test insert, search, etc.
