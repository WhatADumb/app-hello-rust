use lib_hello::{hello_and_bye, saying_bye, saying_hello};

fn main() {
    let hello = saying_hello("Fadhil");
    println!("{}", hello);

    let farewall = saying_bye("Firmansyah");
    println!("{}", farewall);

    hello_and_bye("Fadhil Firmansyah");

    hello::hello()
}

#[test]
fn test_uuid() {
    let uuid = uuid::Uuid::new_v4().to_string();
    println!("ID = {}", uuid);
}

#[test]
fn test_workspace() {
    bye::good_bye();
}