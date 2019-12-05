use xmcd_rs;

fn main() {
    println!("Hello, world!");
    xmcd_rs::xas::foo();
    let file = std::fs::File::open("data/Fe-1_20161207215237.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let arr = xmcd_rs::xas::Xas::load_from_file(reader).unwrap();
    println!("{:?}", arr);
}
