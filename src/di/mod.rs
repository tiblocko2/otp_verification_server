use std::sync::Arc;

use crate::app::query::otp::{get_otp::GetOtpQuery, verify_otp::VerifyOtpQuery};
use crate::storage::repository::OtpRepository;

pub struct Container {
    pub get_otp_query: GetOtpQuery,
    pub verify_otp_query: VerifyOtpQuery,
}

impl Container {
    pub fn new(repository: Arc<dyn OtpRepository>) -> Self {
        Self {
            get_otp_query: GetOtpQuery::new(repository.clone()),
            verify_otp_query: VerifyOtpQuery::new(repository),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        di::Container,
        storage::inmemory::InMemoryOtpRepository,
    };

    /// Проверяем, что DI-контейнер корректно собирается
    #[tokio::test]
    async fn container_is_constructed() {
        // Given
        let repo = Arc::new(InMemoryOtpRepository::new());

        // When
        let container = Container::new(repo);

        // Then
        // Если код компилируется и поля доступны — DI корректен
        let _ = &container.get_otp_query;
        let _ = &container.verify_otp_query;
    }
}
