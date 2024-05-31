// Ref: https://github.com/awslabs/aws-lambda-rust-runtime?tab=readme-ov-file#example-function
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(event: LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, Error> {
    let (event, _context) = event.into_parts();
    let val = event["key"].as_str().unwrap();

    let res = json!({
        "key": val
    });

    Ok(res)
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[tokio::test]
    async fn test_handler() {
        let input = json!({
            "key": "val"
        });
        let context = lambda_runtime::Context::default();
        let event = lambda_runtime::LambdaEvent::new(input.to_owned(), context);

        assert_eq!(input, handler(event).await.unwrap())
    }
}
