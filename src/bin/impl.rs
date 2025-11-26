
fn main(){
 let my_user = User {
    username : String::from("abc_10"),
    name : String::from("abcd__12"),
    email : String::from("abc@gmail.com")
 };

 my_user.print_info();
 
 
}
struct User{
    username : String,
    name : String,
    email : String,
}

impl User {
   fn print_info(&self){
       println!("{}", self.username)
   }
}
