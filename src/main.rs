pub mod service;

fn main() {
    service::initiate_db();
    println!("Done!");
}
