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
        self.repo.save(entry).await;
    }
}

