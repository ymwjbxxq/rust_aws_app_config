use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use rust_aws_app_config::{dtos::my_config::MyConfig, error::ApplicationError};
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<HashMap<String, MyConfig>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize service
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda_runtime::run(handler_fn(|event: serde_json::Value, ctx: Context| {
        execute(event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(_event: serde_json::Value, _ctx: Context) -> Result<(), Error> {
    // ASSUME that this profile is coming from some logic and not from the env variable
    let appconfig_profile = std::env::var("AWS_APPCONFIG_PROFILE").expect("AWS_APPCONFIG_PROFILE must be set");
    if CONFIG.lock().unwrap().contains_key(&appconfig_profile) {
      log::info!("CONFIG FROM HASHMAP{:?}", CONFIG.lock().unwrap().get(&appconfig_profile).unwrap());
    } else {
      let config = fetch_config(&appconfig_profile).await?;
      log::info!("ADDED INTO HASHMAP{:?}", appconfig_profile);
      CONFIG.lock().unwrap().insert(appconfig_profile, config);
    }

    Ok(())
}

async fn fetch_config(appconfig_profile: &str) -> Result<MyConfig, ApplicationError> {
    let appconfig_name = std::env::var("APP_CONFIG_NAME").expect("APP_CONFIG_NAME must be set");
    let appconfig_env = std::env::var("AWS_APPCONFIG_ENVIRONMENT").expect("AWS_APPCONFIG_ENVIRONMENT must be set");
    let appconfig_port = std::env::var("AWS_APPCONFIG_EXTENSION_HTTP_PORT").expect("AWS_APPCONFIG_EXTENSION_HTTP_PORT must be set");

    let url = format!("http://localhost:{}/applications/{}/environments/{}/configurations/{}", appconfig_port, appconfig_name, appconfig_env, appconfig_profile);
    log::info!("URL {:?}", url);
    let response = reqwest::get(url)
        .await?
        .json::<MyConfig>()
        .await?;

    Ok(response)
}
