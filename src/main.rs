use btleplug::api::{Central as _, Manager as _, Peripheral};
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

            // Check if the device matches your phone's name or address
            if let Some(name) = device.properties().await.unwrap().unwrap().local_name {
                if name.contains("Galaxy A10e") {
                    // Replace with your phone's name
                    println!("Connecting to device: {:?}", name);
                    device.connect().await.unwrap();
                    // TODO: if this unwrap fails with the following error:
                    // Other(DbusError(D-Bus error: br-connection-unknown
                    // (org.bluez.Error.Failed)))
                    // the device needs to be paired with the host first
                    println!("Connected to device: {:?}", name);
                    // You can now send and receive messages here

                    // Send an SMS message
                    send_sms(&device, "Hello from Rust!").await;
                }
            }
        }
    }
}

async fn send_sms(device: &impl Peripheral, message: &str) {
    // Here you would implement the logic to send an SMS message
    // This typically involves using the Message Access Profile (MAP)
    // Note: Actual implementation may vary based on device capabilities

    println!("Sending SMS: {}", message);
    // Example: You would need to send the message using the appropriate Bluetooth commands
    // This is a placeholder for the actual implementation
}
