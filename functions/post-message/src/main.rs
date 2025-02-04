use aws_lambda_events::{
    encodings::Body,
    event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse},
};
use serde_json::Value;
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();

    let func = service_fn(post_message_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

fn response(status_code: i64, body: String) -> Result<ApiGatewayProxyResponse, Error> {
    Ok(ApiGatewayProxyResponse {
        status_code,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(body)),
        is_base64_encoded: false,
    })
}

pub(crate) async fn post_message_handler(
    event: LambdaEvent<Value>,
) -> Result<ApiGatewayProxyResponse, Error> {
    log::debug!("Entering handler...");
    log::debug!("Received event: {:?}", event);

    // let Some(text) = event.payload.body else {
    //     return response(400, "No message provided".to_string());
    // };
    //
    // let Ok(uri) = env::var("MONGODB_URI") else {
    //     return response(500, "Database connection error".to_string());
    // };
    //
    // log::debug!("Connecting to MongoDB at {}", uri);
    //
    // let Ok(client) = Client::with_uri_str(uri).await else {
    //     return response(500, "Database connection error".to_string());
    // };
    //
    // let database = client.database("anonmsg");
    // let messages: Collection<Document> = database.collection("messages");
    //
    // if messages
    //     .insert_one(doc! {
    //         "text": text,
    //         "timestamp": Utc::now().timestamp()
    //     })
    //     .await
    //     .is_err()
    // {
    //     return response(500, "Failed to post message".to_string());
    // }

    response(200, "Successfully posted message".to_string())
}
