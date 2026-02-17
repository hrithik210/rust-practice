use dotenv::dotenv;
use std::env;

fn main(){
    dotenv().ok();
    let secret = env::var("secret_code");

    match secret {
        Ok(s) => println!("{}" , s),
        Err(e) => println!("cannot find the env credentials {}" , e)
    }
}