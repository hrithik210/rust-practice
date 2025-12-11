
struct User {
    name  : String,
    email : String,
}

impl User  {
    fn print_info(&self){
        println!("name is {} and email is {}", self.name, self.email);
    }
}
fn main(){
 let user : User = User {
    name  : String::from("hrithik"),
    email : String::from("hrithik@gmail.com"),
 };

 user.print_info();
}