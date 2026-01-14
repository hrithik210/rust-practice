use std::vec;
#[derive(Debug)]
enum BookStatus {
    Availabe,
    CheckedOut(String)
}

struct Book {
    title : String,
    status : BookStatus,
}

impl Book {
    fn checkout(&mut self, name : &String) -> Result<(), String>{
        match &self.status {
            BookStatus::Availabe => {
                println!("{} is available", &self.title);
                self.status = BookStatus::CheckedOut(name.to_string());
                return Ok(());

            },
            BookStatus::CheckedOut(borrower) => {
                println!("{} is already bought by {}", &self.title , borrower);
                Err(format!("{} is already bought by {}", &self.title ,borrower))
            }
    }
}}


fn main(){
    let mut books = vec![Book{
        title : "idk".to_string(),
        status: BookStatus::Availabe,
    }];
    let person = String::from("hrithik");
    match books[0].checkout(&person) {
        Ok(_) => println!("checked out successfully"),
        Err(e) => println!("{}" , e)
    }
}