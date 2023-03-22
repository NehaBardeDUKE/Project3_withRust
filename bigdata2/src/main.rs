use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Request {
    stmt: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    bag_of_words: HashMap<String, usize>,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let result = bigdata::get_main().await?;
    let stmt = event.payload.stmt;
    let bag_of_words = bigdata2::tokenize(stmt.as_str());
    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        bag_of_words,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
