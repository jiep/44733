use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use std::env;
use simple_logger::SimpleLogger;
use lottery::{finder::lottery::Lottery};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_utc_timestamps().with_level(LevelFilter::Info).init().unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {

    const DEFAULT_DRAW_ID: u32 = 1222809102;

    let env_var = env::var("DRAW_ID").is_ok();
    log::warn!("Env var is set: {}", env_var);

    let draw_id: u32 = match env::var("DRAW_ID") {
        Ok(v) => v.parse::<u32>().unwrap(),
        Err(_) => DEFAULT_DRAW_ID
    };

    log::warn!("draw_id: {}", draw_id);

    let number: u32 = event
        .query_string_parameters.first("number").unwrap_or("00000").parse::<u32>().unwrap();

    let lottery = Lottery::load_from_draw_id(draw_id, number).unwrap();
    let r = lottery.parse_to_json().unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(String::from(r))),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}