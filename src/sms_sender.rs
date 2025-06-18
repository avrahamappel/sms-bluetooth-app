pub trait SmsSender {
    fn send_sms(&self, phone_number: &str, message: &str) -> Result<(), String>;
}

