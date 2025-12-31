use std::f32::consts::PI;
#[derive(Debug)]


enum Shape {
 Square(f32),
 Circle(f32),
 Rectangle(f32,f32)

}

fn calcualate_area(s : &Shape) -> f32 {
    let val = match  s{
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width,  height) => width + height
    };
    return val;
}

fn main(){
    let circle = Shape::Circle(22.3);
    let square = Shape::Square(22.3);
    let rectangle = Shape::Rectangle(22.3, 33.2);

    let area_Circle = calcualate_area(&circle);
    let area_square = calcualate_area(&square);

    println!("{area_Circle} {:?}", circle);
    println!("{area_square} {:?}", square);


}