use std::f32::consts::PI;
#[derive(Debug)]


enum Shape {
 Square(f32),
 Circle(f32),
 Rectangle(f32,f32)

}

impl Shape {
    fn calcualate_area(&self) -> f32{
        match &self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width + height
        }
    }
}

fn main(){
    let circle = Shape::Circle(22.3);
    let square = Shape::Square(22.3);
    let rectangle = Shape::Rectangle(22.3, 33.2);

    println!("{} {:?}",circle.calcualate_area() ,circle);
    println!("{} {:?}", square.calcualate_area(),  square);
    println!("{} {:?}", rectangle.calcualate_area(),  rectangle);


}