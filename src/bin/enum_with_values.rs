#[derive(Debug)]
enum Shape {
    Square (f32),
    Circle(f32),
    Rectangle(f32,f32)
}

fn main(){
 let shape_square = Shape::Square(23.3);
 let shape_circle = Shape::Circle(23.3);
 let shape_rectangle = Shape::Rectangle(23.3, 32.1);

 println!("{:?} {:?}", shape_square, shape_circle)

}