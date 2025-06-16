use btleplug::api::{Central as _, Manager as _, Peripheral as _};
use btleplug::platform::Manager;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the Bluetooth manager
    let manager = Manager::new().await.unwrap();

    // Get the list of available adapters
    let adapters = manager.adapters().await.unwrap();

    // Print the available adapters
    for adapter in adapters {
        println!("Found adapter: {:?}", adapter);
    }
}
