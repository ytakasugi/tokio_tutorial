use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::net::TcpStream;
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

pub async fn process(socket: TcpStream, db: Db) {
    // `Connection` を使うことで、バイト列ではなく、Redis の
    // 「フレーム」を読み書きできるようになる
    // この `Connection` 型は mini-redis で定義されている
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                // `Vec<u8>`として保存する
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` はデータが Bytes` 型であることを期待する
                    // この型についてはのちほど解説する
                    // とりあえずここでは `.into()` を使って `&Vec<u8>` から `Bytes` に変換する
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}