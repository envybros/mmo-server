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
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // 데이터 저장용 hashmap
    let mut db = HashMap::new();

    // mini-redis가 제공하는 connection은 이렇게 구문 분석 프레임을 처리한다.
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 클라이언트의 응답
        connection.write_frame(&response).await.unwrap();
    }
}