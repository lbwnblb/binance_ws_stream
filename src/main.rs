use serde_json::json;
use tungstenite::{connect};
fn main() {

    match connect("wss://stream.binance.com:9443"){
        Ok((mut socket, response)) => {
            println!("响应: {:?}", response)
        },
        Err(e) => {
            println!("连接异常: {}", e);
        }
    };
}
