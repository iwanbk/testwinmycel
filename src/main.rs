#[link(name = "winmycelium.dll", kind = "dylib")]
extern "C" {
    fn generate_secret_key() -> Vec<u8>;
}
fn main() {
    unsafe {
        generate_secret_key();
    }
}
