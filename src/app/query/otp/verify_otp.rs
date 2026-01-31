use std::sync::Arc;

use crate::domain::otp::OtpStatus;
use crate::storage::repository::OtpRepository;

pub struct VerifyOtpQuery {
    repo: Arc<dyn OtpRepository>,
}

impl VerifyOtpQuery {
    pub fn new(repo: Arc<dyn OtpRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, phone: String, code: String) -> OtpStatus {
        match self.repo.find(&phone) {
            None => OtpStatus::InvalidCode,
            Some(entry) if entry.is_expired() => {
                self.repo.remove(&phone);
                OtpStatus::Expired
            }
            Some(entry) if entry.code == code => {
                self.repo.remove(&phone);
                OtpStatus::Verified
            }
            Some(_) => OtpStatus::InvalidCode,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::Arc,
        time::{Duration, Instant},
    };

    use crate::{
        app::query::otp::verify_otp::VerifyOtpQuery,
        domain::otp::{OtpEntry, OtpStatus},
        storage::inmemory::InMemoryOtpRepository,
        storage::repository::OtpRepository
    };

    /// Проверяем успешное подтверждение OTP
    #[tokio::test]
    async fn verify_valid_otp() {
        // Given
        let repo = Arc::new(InMemoryOtpRepository::new());
        let entry = OtpEntry::new(
            "79990000000".into(),
            "123456".into(),
            Duration::from_secs(60),
        );
        repo.save(entry);

        let query = VerifyOtpQuery::new(repo.clone());

        // When
        let result = query
            .execute("79990000000".into(), "123456".into())
            .await;

        // Then
        assert_eq!(result, OtpStatus::Verified);
        assert!(repo.find("79990000000").is_none(), "OTP должен быть удалён");
    }

    /// Проверяем реакцию на неверный код
    #[tokio::test]
    async fn invalid_code_returns_error() {
        // Given
        let repo = Arc::new(InMemoryOtpRepository::new());
        repo.save(OtpEntry::new(
            "79990000000".into(),
            "123456".into(),
            Duration::from_secs(60),
        ));

        let query = VerifyOtpQuery::new(repo);

        // When
        let result = query
            .execute("79990000000".into(), "000000".into())
            .await;

        // Then
        assert_eq!(result, OtpStatus::InvalidCode);
    }

    /// Проверяем поведение при истекшем OTP
    #[tokio::test]
    async fn expired_code_returns_expired() {
        // Given
        let repo = Arc::new(InMemoryOtpRepository::new());
        let mut entry = OtpEntry::new(
            "79990000000".into(),
            "123456".into(),
            Duration::from_secs(60),
        );
        entry.expires_at = Instant::now() - Duration::from_secs(1);
        repo.save(entry);

        let query = VerifyOtpQuery::new(repo);

        // When
        let result = query
            .execute("79990000000".into(), "123456".into())
            .await;

        // Then
        assert_eq!(result, OtpStatus::Expired);
    }
}
