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

        // Start scanning for devices
        adapter.start_scan().await.unwrap();
        println!("Scanning for devices...");

        // Wait for a few seconds to discover devices
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Get the discovered devices
        let devices = adapter.peripherals().await.unwrap();
        for device in devices {
            println!("Discovered device: {:?}", device);
        }
    }
}
