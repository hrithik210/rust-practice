
enum Role {
    Admin,
    Regular,
    Pro
}

struct User{
    name : String,
    email : String,
    role : Role
}

impl User {
    fn print_info(&self){

        match self.role {
            Role::Admin => println!("user is a admin"),
            Role::Pro => println!("user is a pro"),
            Role::Regular => println!("user is a Regular")
 
        }

        println!("name of user:{} email : {}", self.name, self.email)
    }
}


fn main(){
    let user : User = User { 
        name: String::from("hrithik"), 
        email: String::from("hrithik@gmail.com"), 
        role: Role::Pro 
    };

    user.print_info();
}