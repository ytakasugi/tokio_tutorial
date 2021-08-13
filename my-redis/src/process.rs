use tokio::net::TcpStream;
use mini_redis::{Connection, Frame};

pub async fn process(socket: TcpStream) {
    // `Connection` を使うことで、バイト列ではなく、Redis の
    // 「フレーム」を読み書きできるようになる
    // この `Connection` 型は mini-redis で定義されている
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}