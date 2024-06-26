use mini_redis::{client, Result};

// 이 녀석은 매크로이다.
#[tokio::main]
async fn main() -> Result<()> {
    // mini-redis 주소에 대한 연결
    // 이런 비동기 함수(connect())는 await가 붙지 않으면 절대 실행되지 않는다.
    // rust에서는... 비동기가 엄청 특이하게 동작한다.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 키, 값 설정
    client.set("envy", "bros".into()).await?;

    // 키 얻어오기
    let result = client.get("envy").await?;

    println!("서버로부터 값을 가져오기; result={:?}", result);

    Ok(())
}