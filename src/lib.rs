pub mod rkdt {
    use std::marker::PhantomData;
    pub trait Point {
        type V: PartialOrd;
        // Returns the number of dimensions, k
        fn size(&self) -> usize;
        // For pos = (0 to k-1) returns a value
        fn at(&self, pos: usize) -> Self::V;
    }
    
    pub struct Vec2 {
        x: f32,
        y: f32
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

    pub struct Vec3 {
        x: f32,
        y: f32,
        z: f32
    }

    // TODO: Implement Vec2
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
        // TODO: implement search, insert and delete
        // TODO: add special root node type? (maybe using newtype pattern)
    }
    
    /*
    // Here's the python code for what this method should do:
    def kdtree(point_list, depth=0):
        try:
            k = len(point_list[0]) # assumes all points have the same dimension
        except IndexError as e: # if not point_list:
            return None
        # Select axis based on depth so that axis cycles through all valid values
        axis = depth % k
     
        # Sort point list and choose median as pivot element
        point_list.sort(key=itemgetter(axis))
        median = len(point_list) // 2 # choose median
     
        # Create node and construct subtrees
        return Node(
            location=point_list[median],
            left_child=kdtree(point_list[:median], depth + 1),
            right_child=kdtree(point_list[median + 1:], depth + 1)
        )
    */
    //TODO: sort floats with sort_by(), instead of sort()
    pub fn from_slice<T: Point + Clone>(points: &[T]) -> Node<T> {
        // TODO: implement creation of KDTree from slice
        return Node{value: points[0].clone(), left: None, right: None};
    }
}


#[test]
fn it_works() {
    let v = vec!(Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0}, Vec2{x: 3.0, y: 1.0});
    let kdt = rkdt::from_slice(v[..]);
}
