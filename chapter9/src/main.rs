use chapter9::errors;

fn main() {
    errors::basics::open_file();
    errors::basics::closure_file();
    errors::basics::expect();

    // this will handle the error
    match errors::basics::propogate() {
        Ok(username) => println!("username = {}", username),
        Err(error) => println!("Error = {}", error)
    }

    // this will panic the error
    let username = errors::basics::propogate().expect("coudn't find username file");
    println!("username = {}", username);

    let username2 = errors::basics::propogate_shorthand().expect("the propogate shorthand failed!!!");
    println!("username = {}", username2);
}
