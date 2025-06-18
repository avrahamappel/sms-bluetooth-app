use crate::sms_sender::SmsSencer;

pub struct BluezSmsSender;

impl SmsSencer for BluezSmsSender {
    fn send_sms(&self, phone_number: &str, message: &str) -> Result<(), String> {
        // Implement the logic to send SMS using BlueZ
        println!("Sending SMS to {}: {}", phone_number, message);
        // Here you would add the actual BlueZ implementation
        Ok(())
    }
}
