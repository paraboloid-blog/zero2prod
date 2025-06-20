pub mod authentication;
pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[cfg(test)]
mod tests {
    use crate::routes::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
