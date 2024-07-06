use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use lottery::finder::lottery::Lottery;
use simple_logger::SimpleLogger;
use std::env;

mod consts;
use crate::consts::{
    DEFAULT_DRAW_ID, DEFAULT_NUMBER_AS_STRING, DRAW_ID_ENV_VAR_NAME, QUERYPARAM_NAME,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn func(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (event, _context) = event.into_parts();

    let env_var = env::var(DRAW_ID_ENV_VAR_NAME).is_ok();
    log::warn!("Env var is set: {}", env_var);

    let draw_id: u32 = match env::var(DRAW_ID_ENV_VAR_NAME) {
        Ok(v) => v.parse::<u32>().unwrap(),
        Err(_) => DEFAULT_DRAW_ID,
    };

    log::warn!("draw_id: {}", draw_id);

    let number: u32 = event
        .query_string_parameters
        .first(QUERYPARAM_NAME)
        .unwrap_or(DEFAULT_NUMBER_AS_STRING)
        .parse::<u32>()
        .unwrap();

    let lottery = Lottery::load_from_draw_id(draw_id, number).unwrap();
    let r = lottery.parse_to_json().unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(r)),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
