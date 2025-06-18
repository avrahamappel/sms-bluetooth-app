use btleplug::api::{Central as _, Manager as _, Peripheral};
use btleplug::platform::Manager;

mod sms_sender;

#[cfg(target_os = "linux")]
mod bluez_sms_sender;

#[cfg(target_os = "macos")]
mod core_bluetooth_sms_sender;

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
    // This is a conceptual placeholder for sending an SMS over MAP
    // You would need to establish a connection to the MAP service

    // Example: Connect to the MAP service (this is pseudo-code)
    let map_service_uuid = "0000181C-0000-1000-8000-00805F9B34FB"; // Example UUID for MAP
    let map_characteristic_uuid = "00002A3E-0000-1000-8000-00805F9B34FB"; // Example UUID for MAP characteristic

    // Connect to the MAP service
    let map_service = device.get_service(map_service_uuid).await.unwrap();
    let map_characteristic = map_service
        .get_characteristic(map_characteristic_uuid)
        .await
        .unwrap();

    // Create a new message (pseudo-code)
    let create_message_command = format!("CREATE_MESSAGE: {}", message);
    map_characteristic
        .write(create_message_command.as_bytes())
        .await
        .unwrap();

    // Notify the user
    println!("SMS sent: {}", message);
}
