use serde_json::json;
use tungstenite::{connect, Message, Utf8Bytes};
fn main() {
    let subscribe = json!({
                "method": "SUBSCRIBE",
                "params": ["btcusdt@trade"],
                "id": 1
            }).to_string();
    match connect("wss://stream.binance.com:9443/ws"){
        Ok((mut socket, response)) => {
            if response.status().as_u16() == 101 {
                match socket.send(Message::Text(Utf8Bytes::from(subscribe))){
                    Ok(_) => {
                        loop {
                            match socket.read() {
                                Ok(Message::Text(text)) => {
                                    println!("{}", text);
                                },
                                Ok(_) => {},
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                }
            };


        },
        Err(e) => {
            println!("连接异常: {}", e);
        }
    };
}
