use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // mini-redis 주소에 대한 연결
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 키, 값 설정
    client.set("envy", "bros".into()).await?;

    // 키 얻어오기
    let result = client.get("envy").await?;

    println!("서버로부터 값을 가져오기; result={:?}", result);

    Ok(())
}