use proxy::proxy;

#[proxy]
fn main() {
    println!("Hello, world!");
    proxy_sys::exports::initialize(0 as winapi::shared::minwindef::HINSTANCE).unwrap_or_else(|e| {
        std::panic!("{}", e);
    });
}