use mini_redis::{client, Result as MiniResult};

#[tokio::main]
async fn main() -> MiniResult<()> {
    // 建立与mini-client服务器的连接
    let mut client = client::connect("127.0.0.1:8888").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("get from server: {:?}", result);
    Ok(())
}

// #[tokio::main] 转换async fn main为main的同时还对异步运行时进行了初始化 上述代码转换成了类似如下
/*fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _res = rt.block_on(async {
        let mut client = client::connect("127.0.0.1:8888").await.map_err(|err| {
            eprintln!("error: {:?}", err);
            "connect redis server error".to_string()
        })?;
        let _ = client.set("hello", "world".into()).await.map_err(|err| {
            eprintln!("error: {:?}", err);
            "client set redis value error".to_string()
        })?;
        let result = client.get("hello").await.map_err(|err| {
            eprintln!("error: {:?}", err);
            "client set redis value error".to_string()
        })?;
        println!("get from server: {:?}", result);
        Ok::<(), String>(())
    });
}*/
