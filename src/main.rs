use serde_json::json;
use tungstenite::{connect};
fn main() {

    match connect("stream.binance.com"){
        Ok((mut socket, response)) => {
            println!("响应: {:?}", response)
        },
        Err(e) => {
            println!("连接异常: {}", e);
        }
    };
}
