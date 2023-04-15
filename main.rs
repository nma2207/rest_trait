use std::time::Instant;

pub trait Area {
    fn get_area(&self) -> f32;
}

pub struct Square {
    pub side: f32
}

impl Area for Square {
    fn get_area(&self)->f32 {
        self.side * self.side
    }
}

pub struct Triangle{
    pub w: f32,
    pub h: f32
}

impl Area for Triangle{
    fn get_area(&self)->f32 {
        0.5 * self.w * self.h
    }
}

struct Rectangle{
    pub a: f32,
    pub b: f32
}

impl Area for Rectangle{
    fn get_area(&self)->f32 {
        self.a * self.b
    }
}

struct Circle{
    pub r: f32,
}

impl Area for Circle{
    fn get_area(&self)->f32 {
        3.14 * self.r * self.r
    }
}

fn get_area1<T>(shapes: &Vec<Box<T>>) -> f32
where T: Area + ?Sized {
    shapes.iter().fold(0.0,  | sum, shape| sum + shape.get_area())
}

enum ShapeType{
    Square,
    Triangle,
    Rectangle,
    Circle
}

struct Shape2{
    pub _type: ShapeType,
    pub a: f32,
    pub b: f32
}

impl Shape2{
    fn get_area(&self)->f32 {
        match self._type {
            ShapeType::Square=> self.a * self.a,
            ShapeType::Triangle => self.a * self.b * 0.5,
            ShapeType::Rectangle=> self.a * self.b,
            ShapeType::Circle=> 3.14 * self.a * self.a
        }
    }
}

enum Shape3{
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32)
}

impl Area for Shape3{
    fn get_area(&self) -> f32 {
        match self{
            Shape3::Square(a) => a*a,
            Shape3::Rectangle(a, b) => a*b,
            Shape3::Triangle(a, b) => 0.5 *a*b,
            Shape3::Circle(a) => 3.14 * a* a
        }


    }
}
fn get_area2(shapes: &Vec<Shape2>) -> f32
{
    shapes.iter().fold(0.0,  | sum, shape| sum + shape.get_area())
}
fn get_area3(shapes: &Vec<Shape3>) -> f32
{
    shapes.iter().fold(0.0,  | sum, shape| sum + shape.get_area())
}
fn main() {
    let shapes: Vec<Box<dyn Area>> = vec![Box::new(Triangle{w: 1.123, h:31.2}),
                                          Box::new(Rectangle{a:55.00, b:11.00}),
                                          Box::new(Circle{r: 5.0}),
                                          Box::new(Square{side: 12.0})];
    let shapes2 = vec![Shape2{_type:ShapeType::Triangle, a: 1.1, b: 2.0},
                       Shape2{_type:ShapeType::Rectangle, a: 0.5, b: 0.5},
                       Shape2{_type:ShapeType::Circle, a: 3.14, b: 3.14},
                       Shape2{_type:ShapeType::Square, a: 12.1, b: 12.1}];
    let shapes3 = vec![Shape3::Triangle(1.1, 2.0),
                       Shape3::Rectangle(0.5, 0.5),
                       Shape3::Circle(3.14),
                       Shape3::Square(12.1)];

    let n = 1000000;

    let start = Instant::now();
    for _ in 1..n+1 {
        get_area1(&shapes);
    }
    let duration = start.elapsed();
    println!("{} ns", duration.as_nanos());

    let start = Instant::now();
    for _ in 1..n+1 {
       get_area2(&shapes2);
    }
    let duration = start.elapsed();
    println!("{} ns", duration.as_nanos());

    let start = Instant::now();
    for _ in 1..n+1 {
        get_area3(&shapes3);
    }
    let duration = start.elapsed();
    println!("{} ns", duration.as_nanos());
}
