use tokio::net::TcpListener;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod process;
use crate::process::process;

#[tokio::main]
async fn main() {
    // リスナーをこのアドレスにバインドする
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // タプルの2つ目の要素は、新しいコネクションのIPとポートの情報を含んでいる
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        println!("Accepted");
        // それぞれのインバウンドソケットに対して新しいタスクを spawn する
        // ソケットは新しいタスクに move され、そこで処理される
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}