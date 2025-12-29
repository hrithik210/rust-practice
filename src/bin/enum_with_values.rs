#[derive(Debug)]
enum Shape {
    square (f32),
    circle(f32),
    rectangle(f32,f32)
}

fn main(){
 let shape_square = Shape::square(23.3);
 let shape_circle = Shape::circle(23.3);
 let shape_rectangle = Shape::rectangle(23.3, 32.1);

 println!("{:?} {:?}", shape_square, shape_circle)

}