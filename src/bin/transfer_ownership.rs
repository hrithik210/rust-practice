



fn main(){
 let a = String::from("hrithik");
 let length  = get_length(a);

 println!("{}" , length);
}

fn get_length(s: String) -> usize {
 return  s.len();
}