mod utils;
//mod number;

trait Shape<T> {
    fn area(&self) -> T;
}

struct Point {
    x: f32,
    y: f32,
}

struct Square {
    top_left: Point,
    side_length: f32,
}

impl Square {
    fn print_top_left(&self) {
        use utils::xy;
        let (x, y) = xy(&self.top_left.x, &self.top_left.y);
        println!("x is {} and y is {}", x, y);
    }
}

impl Shape<f32> for Square {
    fn area(&self) -> f32 {
        &self.side_length * &self.side_length
    }
}

fn main() {
    let top_left = Point { x: 4.123, y: 34.12 };

    let a = Square { top_left, side_length: 53.4 };
    let mut b = a;
    b.side_length = 54.876;
    b.print_top_left();

    let area = b.area();
    println!("area of square: {:?}", area);

    let number = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("what is the number? {}", number);
}

