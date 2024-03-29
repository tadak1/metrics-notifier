mod cloud_watch_metrics_client;
mod error;
mod time_range;
mod metric;
mod ec2_instance_client;

use lambda::{handler_fn, Context};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct ReportEvent {}

#[derive(Serialize)]
pub struct ReportHandlerOutput {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    lambda::run(handler_fn(report_handler)).await?;
    Ok(())
}

async fn report_handler(
    event: Value,
    _: Context,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Ok(event)
}
