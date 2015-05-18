mod vec2;
mod vec3;

pub use vec2::Vec2;
pub use vec3::Vec3;

/*
The `Point` trait outlines the properties that a value must provide to be indexed in the k-d tree.
See the `Vec2` and `Vec3` structs for an example implementation.
*/
pub trait Point: PartialEq + Clone + Debug {
    type V: PartialOrd + PartialEq + Copy + Debug;
    // Returns the number of dimensions, k
    fn size(&self) -> usize;
    // For pos = (0 to k-1) returns a sub-value for this point.
    fn at(&self, pos: usize) -> Self::V;
}

/*
The `Node` struct encompasses a single k-d tree node, storing a value as well as optional 
heap-pointers to the next subnodes in the tree (`left` and `right`).

TODO: Try to eliminate Boxes by introducing more static parmeters as in from_slice()?
*/
pub struct Node<T: Point> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>
}

impl<T: Point> Node<T> {
    // Get the node's Point value
    pub fn value(&self) -> &T {
        &self.value
    }
    
    // Insert a new value into the tree
    pub fn insert(&mut self, value: &T) {
        self.insert_node(value, 0, value.size());
    }
    
    /*
    Recursively search for a position to insert a new value into the tree.
    Warning: May unbalance the tree
    */
    fn insert_node(&mut self, value: &T, axis: usize, k: usize) {
        if *value == self.value {
            return;
        } else if value.at(axis) < self.value.at(axis) {
            match self.left {
                Some(ref mut n) => n.insert_node(value, (axis + 1) % k, k),
                None => self.left = Some(Box::new(Node{value: value.clone(), left: None, right: None}))
            }
        } else if value.at(axis) >= self.value.at(axis) {
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

use std::fmt::{Debug, Formatter, Error};
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

// Create a new k-d tree from a slice of point values.
pub fn from_slice<T: Point>(points: &[T]) -> Option<Box<Node<T>>> {
    let mut points_clone = points.to_vec();
    create_nodes(&mut points_clone[..], 0)
}

/*
Build a k-d tree from a slice of points. Algorithm taken from:
http://en.wikipedia.org/wiki/K-d_tree
*/
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

#[test]
fn test_from_slice() {
    let v = vec!(Vec2{x: 2.0, y: 3.0}, Vec2{x: 5.0, y: 4.0}, Vec2{x: 9.0, y: 6.0}, 
            Vec2{x: 4.0, y: 7.0}, Vec2{x: 8.0, y: 1.0}, Vec2{x: 7.0, y: 2.0});
    let kdt = from_slice(&v[..]).unwrap();
    println!("{:?}", kdt);
    println!("{:?} == {:?}", kdt.value, v[5]);
    assert!(kdt.value == v[5]);
    // This is just an easy way to check the tree is as expected without walking the whole thing.
    assert!(format!("{:?}", kdt) == "[(7,2), [(5,4), [(2,3)], [(4,7)]], [(9,6), [(8,1)]]]");
}

#[test]
fn test_insert() {
    let v = vec!(Vec2{x: 2.0, y: 3.0}, Vec2{x: 5.0, y: 4.0}, Vec2{x: 9.0, y: 6.0}, 
            Vec2{x: 4.0, y: 7.0}, Vec2{x: 8.0, y: 1.0}, Vec2{x: 7.0, y: 2.0});
    let mut kdt = from_slice(&v[..]).unwrap();
    println!("{:?}", kdt);
    kdt.insert(&Vec2{x: 1.0, y: 4.0});
    println!("after insert: {:?}", kdt);
    assert!(format!("{:?}", kdt) == "[(7,2), [(5,4), [(2,3)], [(4,7), [(1,4)]]], [(9,6), [(8,1)]]]");
    kdt.insert(&Vec2{x: 5.0, y: 9.0});
    println!("after second insert: {:?}", kdt);
    assert!(format!("{:?}", kdt) == "[(7,2), [(5,4), [(2,3)], [(4,7), [(1,4)], [(5,9)]]], [(9,6), [(8,1)]]]");
}

// TODO: Test insert, search, etc.
