use proxy_dll::proxy;

#[proxy]
fn main() {
    msgbox::create("Hello, world!", "Hello Rust", msgbox::IconType::None).unwrap();
}
