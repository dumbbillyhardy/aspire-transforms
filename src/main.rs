mod protos;
use protos::aspire::{MintTransaction, AspireTransaction};

fn main() {
    // Encode example request
    let mut out_msg = MintTransaction::new();
    out_msg.set_description("John Smith".to_string());
    println!("{}", out_msg.get_description());
}
