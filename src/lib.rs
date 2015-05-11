pub mod rkdt {
    pub trait Point {
        type V: PartialOrd + PartialEq + Copy;
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

    // TODO: Try to eliminate Boxes by introducing more static parmeters as in from_slice()?
    pub struct Node<T: Point + PartialEq + Clone> {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>
    }

    impl<T: Point + PartialEq + Clone> Node<T> {
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
    
    /*
    Build a k-d tree from a slice of points. Algorithm taken from:
    http://en.wikipedia.org/wiki/K-d_tree
    */
    pub fn from_slice<T: Point + PartialEq + Clone>(points: &mut [T]) -> Option<Box<Node<T>>> {
        create_nodes(points, 0)
    }
    
    fn create_nodes<T: Point + PartialEq + Clone>(points: &mut [T], depth: usize) -> Option<Box<Node<T>>> {
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
fn it_works() {
    use rkdt::Vec2;
    let mut v = vec!(Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0});
    let kdt = rkdt::from_slice(&mut v[..]);
}
