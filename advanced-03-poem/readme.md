# poem 框架

我最近在重新学rust ，巩固一下细节，也写一个Chaineye rust 教程，供想要学习的小伙伴查看。

推特：@seek_web3

Chainey 社群： 官网 chaineye.info | Chaineye Rust 教程 | 微信: LGZAXE, 加微信之后拉各社群

所有代码和教程开源在github: https://github.com/0xchaineye/chaineye-rust

-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Poem 是一个功能齐全且易于使用的 Web 框架，采用 Rust 编程语言。

## 1.快速开始

```
use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
```

## 2.Endpoint

trait Endpoint表示可以处理 HTTP 请求的类型，它返回该Result<T: IntoResponse, Error>类型。宏handler用于将函数转换为端点。

```
use poem::{
    error::NotFoundError, handler, http::StatusCode, test::TestClient, Endpoint, Request,
    Result,
};

#[handler]
fn return_str() -> &'static str {
    "hello"
}

#[handler]
fn return_err() -> Result<&'static str, NotFoundError> {
    Err(NotFoundError)
}

let resp = TestClient::new(return_str).get("/").send().await;
resp.assert_status_is_ok();
resp.assert_text("hello").await;

let resp = TestClient::new(return_err).get("/").send().await;
resp.assert_status(StatusCode::NOT_FOUND);
```

## 3.Extractors

Extractor 用于从 HTTP 请求中提取一些内容。

Poem提供一些常用的 Extractor，用于从 HTTP 请求中提取内容。

在下面的示例中，该 index 函数使用 3 个 Extractor 来提取远程地址、HTTP 方法和 URI。

```
use poem::{
    handler,
    http::{Method, Uri},
    web::RemoteAddr,
};

#[handler]
fn index(remote_addr: &RemoteAddr, method: Method, uri: &Uri) {}
```

默认情况下，提取器会400 Bad Request在发生错误时返回一个，但有时您可能希望更改此行为，以便您自己处理错误。

在下面的示例中，当Query提取器失败时，它将返回500 Internal Server响应和错误原因

```
use poem::{
    error::ParseQueryError, handler, http::StatusCode, web::Query, IntoResponse, Response,
    Result,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
fn index(res: Result<Query<Params>>) -> Result<impl IntoResponse> {
    match res {
        Ok(Query(params)) => Ok(params.name.into_response()),
        Err(err) if err.is::<ParseQueryError>() => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(err.to_string())),
        Err(err) => Err(err),
    }
}
```

## 4.路由

共有三种可用路由。

Route 路径的路由
RouteDomain域路由
RouteMethodHTTP方法的路由

```
use poem::{get, handler, post, web::Path, Route};

#[handler]
async fn get_user(id: Path<String>) {}

#[handler]
async fn delete_user(id: Path<String>) {}

#[handler]
async fn create_user() {}

let app = Route::new()
    .at("/user/:id", get(get_user).delete(delete_user))
    .at("/user", post(create_user));
```

您可以创建自定义提取器，下面我们会说到

## 5.Responses

所有可以转换为 HTTP 响应的类型都Response应该实现IntoResponse.

在下面的示例中，string_responseandstatus_response 函数返回StringandStatusCode类型，因为Poem已经IntoResponse为它们实现了特征。

该no_response函数不返回值。我们可以认为它的返回类型是()，并且Poem还实现了IntoResponsefor ()，它总是被转换为200 OK。

该result_response函数返回一个Result类型，这意味着可能会发生错误。

```
use poem::{handler, http::StatusCode, Result};

#[handler]
fn string_response() -> String {
    todo!()
}

#[handler]
fn status_response() -> StatusCode {
    todo!()
}

#[handler]
fn no_response() {}

#[handler]
fn result_response() -> Result<String> {
    todo!()
}
```

## 6.处理错误

以下示例在 NotFoundError发生时返回自定义内容。

```
use poem::{
    error::NotFoundError, handler, http::StatusCode, EndpointExt, IntoResponse, Response, Route,
};

#[handler]
fn foo() {}

#[handler]
fn bar() {}

let app =
    Route::new()
        .at("/foo", foo)
        .at("/bar", bar)
        .catch_error(|err: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("custom not found")
        });
```

## 7. 中间件

您可以调用with上的方法Endpoint将中间件应用到端点。它实际上将原始端点转换为新端点。

```
use poem::{handler, middleware::Tracing, EndpointExt, Route};

#[handler]
fn index() {}

let app = Route::new().at("/", index).with(Tracing);
```

您可以创建自己的中间件，下面我们会说到


## 8.板条箱功能

|  Feature	       |              Description          |
|:----------------:|:----------------------------------|
|        server	   |     服务器和监听器 API（默认启用）    |
|     compression	 |     支持解压请求体和压缩响应体        |
|     cookie	     |        支持 Cookie                |
|      csrf	       |      支持跨站请求伪造 (CSRF) 保护    |
|    multipart	   |      支持 multipart               |
|native-tls	       |     支持通过 TLS 的 HTTP 服务器native-tls |
|openssl-tls	     |     支持通过 TLS 的 HTTP 服务器openssl-tls |
|opentelemetry	   |        支持opentelemetry          |
|prometheus	       |         支持普罗米修斯              |
|redis-session	   |        支持 RedisSession          |
|rustls            |	    	支持通过 TLS 的 HTTP 服务器rustls |
|session	         |            支持session        |
|sse	             |           支持服务器发送事件 (SSE)   |
|tempfile	         |        支持tempfile     |
|tower-compat      |	     tower::Layer和的适配器tower::Service  |
|websocket	       |       支持 WebSocket                       |
|anyhow	           |       与anyhow箱子结合                  |
|eyre06	           |       与eyrecrate 的 0.6.x 版本集成。   |
|i18n              |	      支持国际化                          |
|acme-native-roots |   支持 ACME（自动证书管理环境）.    |  
|acme-webpki-roots |    	支持使用 webpki TLS 根而非本机 TLS 根的 ACME |
|tokio-metrics	   |    与tokio-metrics箱子结合    |
|embed	           |    与板条箱集成rust-embed      |
|xml	             |    与板条箱集成quick-xml    |
|yaml	             |      与板条箱集成serde-yaml       |

## 9. Re-exports

```
- pub use error::Error;
- pub use error::Result;
- pub use middleware::Middleware;
- pub use web::FromRequest;
- pub use web::IntoResponse;
- pub use web::RequestBody;
```

## 10. Modules

- endpoint: endpoint 相关类型。
- error: 一些常见的错误类型。
- http: 常见 HTTP 类型的通用库
- i18n: 国际化
- listener: 常用的侦听器
- middleware: 常用的中间件
- session: 会话管理
- test: 测试实用程序来测试您的端点
- web: 	通常用作提取器或响应的类型。

## 11. Structs

- Body: 请求和响应的正文对象
- OnUpgrade: 未来可能的 HTTP 升级
- Request: 表示 HTTP 请求
- RequestBuilder: 请求生成器
- RequestParts: HTTP 请求的组成部分
- Response: 表示 HTTP 响应
- ResponseBuilder: 响应生成器
- ResponseParts: 	HTTP 响应的组成部分
- Route: 路由对象
- RouteDomain: HOST标头的路由对象
- RouteMethod: HTTP 方法的路由对象
- RouteScheme: 请求方案的路由对象
- Server: HTTP 服务器
- Upgraded: 升级的 HTTP 连接

## 12.枚举

Addr: 一个网络地址

## 13. Traits

- Endpoint: HTTP 请求处理程序。
- EndpointExt: 的扩展特征Endpoint。
- IntoEndpoint:	表示可以转换为终结点的类型。

## 14. 函数

- connect: 辅助函数，类似于RouteMethod::new().connect(ep).
- delete: 辅助函数，类似于RouteMethod::new().delete(ep).
- get: 辅助函数，类似于RouteMethod::new().get(ep).
- head: 辅助函数，类似于RouteMethod::new().head(ep).
- options: 辅助函数，类似于RouteMethod::new().options(ep).
- patch: 辅助函数，类似于RouteMethod::new().patch(ep).
- post: 辅助函数，类似于RouteMethod::new().post(ep).
- put: 辅助函数，类似于RouteMethod::new().put(ep).
- trace: 辅助函数，类似于RouteMethod::new().trace(ep).

## 15.属性宏

- async_trait
- handler: 	将异步函数包装为Endpoint.

