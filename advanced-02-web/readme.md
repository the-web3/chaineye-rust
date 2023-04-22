# rust web 框架介绍


我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群 

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------

## 1.hyper

Hyper是一个基于Rust语言的Web框架，它提供了一种高性能、安全且易于使用的方式来编写HTTP服务器和客户端。Hyper的目标是提供一个简单而强大的HTTP库，可以与Rust的生态系统无缝集成。

Hyper的设计非常现代化，它支持异步/await语法、实现了HTTP/1和HTTP/2协议、提供了可扩展的中间件机制等。Hyper可以轻松地与其他Rust库和工具集成，例如Tokio异步运行时、serde序列化框架等。

Hyper还提供了一个高级别的web框架hyper-rustacean，它简化了使用Hyper构建Web应用程序的过程。Hyper-rustacean提供了一个类似于Express.js的路由系统，可以轻松地创建RESTful API和Web应用程序。

总的来说，Hyper是一个高性能、安全、易于使用的Web框架，尤其适合需要高性能HTTP服务器的应用程序。它的使用也越来越普及，成为了Rust生态系统中最流行的HTTP库之一。

### 1.hyper 社区资料

- [github 地址](https://github.com/hyperium/hyper)
- [hyper.rs 网站](https://hyper.rs/)

### 2.用 hyper app 登陆注册的案例

#### 代码解释

构建完项目之后，我们在 Cargo.toml 里面写入依赖包
```
[dependencies]
bytes = "1"
hyper = { version = "1.0.0-rc.3", features = ["full"] }
tokio = { version = "1", features = ["full"] }
http-body-util = "0.1.0-rc.2"
serde_json = "1.0"
pretty_env_logger = "0.4"
serde = { version = "1.0.152", features = ["derive"] }
```

编写 app_info, login 和 注册的接口

```
async fn register(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    let whole_body = req.collect().await?.aggregate();
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    data["token"] = serde_json::Value::from("0x000000000000");
    data["msg"] = serde_json::Value::from("register successs");
    // And respond with the new JSON.
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}

async fn login(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    // Aggregate the body...
    let whole_body = req.collect().await?.aggregate();
    // Decode as JSON...
    let mut data: serde_json::Value = serde_json::from_reader(whole_body.reader())?;
    // Change the JSON...
    data["msg"] = serde_json::Value::from("login success");
    // And respond with the new JSON.
    let json = serde_json::to_string(&data)?;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}

async fn app_info() -> Result<Response<BoxBody>> {
    let chaineye_app = AppInfoData {
        name: String::from("chaineye.info"),
        version: String::from("v1.0.1"),
    };
    let json = serde_json::to_string(&chaineye_app).unwrap();
     let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(full(json))?;
    Ok(response)
}
```

服务函数

```
async fn hyper_service(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/register") => register(req).await,
        (&Method::POST, "/login") => login(req).await,
        (&Method::GET, "/app_info") => app_info().await,
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(NOTFOUND))
                .unwrap())
        }
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let addr: SocketAddr = "127.0.0.1:1337".parse().unwrap();

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let service = service_fn(move |req| hyper_service(req));

            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
```
#### 完整版代码编译执行

- [完整版代码](https://github.com/0xchaineye/chaineye-rust/tree/main/advanced-02-web/code/hyper-server)

- 代码构建

```
cd hyper-server
cargo build
```

- target 里面启动项目

```
./hyper-server
```

- 代码测试

```
curl http://127.0.0.1:1337/app_info
```

```
curl --location --request POST 'http://127.0.0.1:1337/register' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "aaa",
    "password": "Qwer1234!"
}'
```

```
curl --location --request POST 'http://127.0.0.1:1337/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "username": "aaa",
    "password": "Qwer1234!"
}'
```


## 2. actix-web


## 3. rocket


## 4.poem


## 5. warp


## 6.nickel


## 7.tide




