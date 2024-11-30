use std::{future::IntoFuture, io};

use actix_web::{body::MessageBody, dev::{self, ServiceRequest, ServiceResponse}, middleware::Next, web::{self, Bytes}, Error, FromRequest};
use tracing::{info, level_filters::LevelFilter, Subscriber};
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Layer};
use futures_util::StreamExt;

/// 初始化log
pub fn log<W>(w: W, env_filter: String) -> impl Subscriber 
where 
    W: for<'writer> MakeWriter<'writer> + 'static,
{
    // 日志——控制台
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(io::stdout)
        .with_filter(LevelFilter::DEBUG);

    // 日志——文件
    let file_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .compact()
        .with_ansi(false)
        .with_writer(w)
        .with_filter(EnvFilter::new(env_filter));

    // 日志等级
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    tracing_subscriber::registry()
        // .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        // .init();
}

/// 日志中间件
pub async fn log_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {

    // 前置处理   
    info!("-----------------------请求开始---------------------------");
    let path = req.path();
    let method = req.method().as_str();

    info!("method: {}, path: {}", method, path);
    let query_string = req.query_string();

    let call = if query_string.len() > 0  {
        // get请求参数
        info!("请求参数: {}", query_string);
        next.call(req).await
    } else {
        // post请求参数
        let (hr, mut payload) = req.into_parts();
        let either = Bytes::from_request(&hr, payload.by_ref());
        let bytes = either.into_future().await?;
        let param = bytes.iter()
            .map(|c| {*c as char})
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        
        info!("请求参数: {}", param);
        let req = ServiceRequest::from_parts(hr, bytes_to_payload(bytes));
        next.call(req).await
    };

    // 后置处理
    info!("-----------------------请求结束---------------------------");

    call
}

/// 获取Payload
fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = actix_http::h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
