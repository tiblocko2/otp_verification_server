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