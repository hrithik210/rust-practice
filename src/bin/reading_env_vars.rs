use dotenv::dotenv;
use std::env;

fn main(){
    dotenv().ok();
    let secret = env::var("secret_code");
    let s = secret.unwrap();
    println!("{}" , s)
}