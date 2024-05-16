mod helper;

#[tokio::main]
async fn main() {
    let runner = helper::init_execute_runner("domain_of_definition_validator.yml");
    let result = runner.start().await;
    assert!(result.is_ok());
}