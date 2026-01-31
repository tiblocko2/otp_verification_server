use std::{sync::Arc, time::Duration};
use rand::{rng, Rng};

use crate::domain::otp::OtpEntry;
use crate::storage::repository::OtpRepository;

pub struct GetOtpQuery {
    repo: Arc<dyn OtpRepository>,
}

impl GetOtpQuery {
    pub fn new(repo: Arc<dyn OtpRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, phone: String) {
        let code = rng().random_range(100000..999999).to_string();
        let entry = OtpEntry::new(phone, code, Duration::from_secs(300));
        self.repo.save(entry);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        app::query::otp::get_otp::GetOtpQuery,
        storage::inmemory::InMemoryOtpRepository,
    };
    use crate::storage::repository::OtpRepository;

    /// Проверяем, что запрос генерации OTP сохраняет код в репозиторий
    #[tokio::test]
    async fn generate_and_store_otp() {
        // Given
        let repo = Arc::new(InMemoryOtpRepository::new());
        let query = GetOtpQuery::new(repo.clone());

        // When
        query.execute("79990000000".into()).await;

        // Then
        let stored = repo.find("79990000000");
        assert!(stored.is_some(), "OTP должен быть сохранён");
    }
}

