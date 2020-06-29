fn usage() {
    let mut the_version = env!("CARGO_PKG_VERSION");
    the_version = "0.2";

    println!("tinymd, a markdown compiler written by Hblee");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
