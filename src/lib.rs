pub mod rkdt {
    pub trait Point {
        type V: PartialOrd + Copy;
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

    // TODO: Try to eliminate Boxes by introducing more static parmeters as in from_slice()?
    pub struct Node<T: Point> {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>
    }

    impl<T: Point> Node<T> {
        pub fn new(val: T, l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) -> Node<T> {
            Node{value: val, left: l, right: r}
        }
        
        pub fn value(&self) -> &T {
            &self.value
        }
        // TODO: implement search, insert and delete
        // TODO: add special root node type? (maybe using newtype pattern)
    }
    
    /*
    Build a k-d tree from a slice of points. Algorithm taken from:
    http://en.wikipedia.org/wiki/K-d_tree
    */
    pub fn from_slice<T: Point + Copy>(points: &mut [T]) -> Option<Box<Node<T>>> {
        create_node(points, 0)
    }
    
    fn create_node<T: Point + Copy>(points: &mut [T], depth: usize) -> Option<Box<Node<T>>> {
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
            value: points[median_index],
            left: create_node(&mut points[..median_index], depth + 1),
            right: create_node(&mut points[(median_index + 1)..], depth + 1)
        }))
    }
}

#[test]
fn it_works() {
    use rkdt::Vec2;
    let mut v = vec!(Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0});
    let kdt = rkdt::from_slice(&mut v[..]);
}
