use crate::domain::otp::OtpEntry;

pub trait OtpRepository: Send + Sync {
    fn save(&self, entry: OtpEntry);
    fn find(&self, phone: &str) -> Option<OtpEntry>;
    fn remove(&self, phone: &str);
}
