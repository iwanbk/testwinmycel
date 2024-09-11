#[link(name = "winmycelium.dll", kind = "dylib")]
extern "C" {
    fn generate_secret_key() -> Vec<u8>;
    fn start_mycelium(peers: Vec<String>, priv_key: Vec<u8>);
}
fn main() {
    unsafe {
        let key = generate_secret_key();
        let peers = vec!["tcp://185.69.166.7:9651".to_string()];
        start_mycelium(peers, key);
    }
}
