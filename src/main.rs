// We always want to run Clippy on this code and have it as a dependency. Don't
// do this for real Rust applications.

extern crate fastrs;

fn main() {
    let port: u16 = 9090;
    let upstream_port: u16 = 80;
    // If a function returns something in Rust you can't ignore it, so we need this superflous
    // unused variable here. Starting it with "_" tells the compiler to ignore it.
    let _listening = fastrs::start_server(port, upstream_port);
}
