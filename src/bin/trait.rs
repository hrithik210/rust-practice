// The Contract


struct User{
    name : String,
    email :  String
}

struct Product{
    name : String,
    price : String
}
pub trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for User {
    fn summarize(&self) -> String {
        format!("this is a user with name {} and email is {}", self.name, self.email)
    }
}

impl Summarizable for Product {
    fn summarize(&self) -> String {
        format!("this is a product with name {} and price is {}", self.name, self.price)
    }
}

fn main(){
    let product = Product{
        name : String::from("phone"),
        price : String::from("$122")
    };

    let user = User{
        name : String::from("hrithik"),
        email:  String::from("hrithik@gmail.com")
    };

    println!("{}", product.summarize());
    println!("{}", user.summarize());

}