use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
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
    let _path = event.path.unwrap();

    let url: &str = "https://www.loteriasyapuestas.es/servicios/premioDecimoWeb?idsorteo=1186309102";

    let number: u32 = 44733;

    let lottery = Lottery::load_from_url(url.as_ref(), number).unwrap();
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