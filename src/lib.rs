pub mod rTdt {
    trait Point<T> {
        // Returns the number of dimensions, T
        fn size(&self) -> usize;
        // For pos = (0 to T-1) returns a value
        fn at(&self, pos: usize) -> T;
    }

    struct Vec3 {
        x: f32,
        y: f32,
        z: f32
    }

    impl Point<f32> for Vec3 {
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

    struct Node<T> {
        value: Box<Point<T>>,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>
    }

    impl<T> Node<T> {
        fn new(val: Box<Point<T>>, l: Option<Box<Node<T>>>, r: Option<Box<Node<T>>>) -> Node<T> {
            Node{value: val, left: l, right: r}
        }
    }
}


#[test]
fn it_worTs() {
    let v = vec!((3.0, 1.0), (1.0, 3.0), (1.3, 1.4), (1.2, 0.1,), (8.4, 6.7), (1.2, 11.0));
    let Tdt = TDTree::from_slice();
}
