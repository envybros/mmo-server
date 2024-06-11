use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // 리스너를 주소에 "바인딩"한다.
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // 두번째 인자에는 새 연결의 ip와 포트가 포함된다.
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // connection을 사용하면 redis frames 대신 byte streams이다.
    // connection 유형은 mini-redis로 정의된다.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // error 반환
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}