use chrono::{Utc , Local};

fn main(){
 let global_time = Utc::now();
 let current_time = Local::now();
 println!("global time - {}" , global_time );
 println!("current time - {}", current_time)

}