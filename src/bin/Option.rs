struct User{
    username : String,
    name : String,
    email : Option<String>
}

impl User{
    fn print_info(&self){

        match self.email{
            Some(email) => println!("username is {} and name is {} email is {}", self.username, self.name, email),
            None => println!("username is {} and name is {}", self.username, self.name)
        }
    }
}

fn main(){
    let user = User{
        username : String::from("hrithik"),
        name: String::from("hrithik"),
        email : Option::None
    };

    user.print_info();
    
}