use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectange {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    //allocate memory on the heap and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let point: Point = origin();
    let rectangle: Rectange = Rectange {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let boxed_rectangle: Box<Rectange> = Box::new(Rectange {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let boxed_point: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
}
