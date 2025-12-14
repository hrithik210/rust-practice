struct User{
    name : String,
    email : String
}


struct Product{
    name : String,
    price: String
}
pub trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for User{
    fn summarize(&self) -> String {
        format!("email of {} is {} ", self.name,self.email)
    }
}


impl Summarizable for Product{
    fn summarize(&self) -> String {
        format!("PRICE of {} is {} ", self.name,self.price)
    }
}

fn print_summary< T : Summarizable >(item : &T){
    println!("{}", item.summarize())
}

fn main(){
    let user = User{
        name : String::from("hrithik"),
        email : String::from("hrithik@gmail.com")
    };

    let product = Product{
        name : String::from("mango"),
        price : String::from("122")
    };

    print_summary(&user);

}