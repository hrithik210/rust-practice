struct User{
    username : String,
    email : String,
    active : bool
}

fn main(){
    let my_user = User {
        username : String::from("hrithik"),
        email : String::from("hrithik@gmail.com"),
        active : true
    };

    println!("{} {} {}", my_user.username , my_user.email, my_user.active)

}

