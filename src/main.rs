use btleplug::api::{AddressType, BDAddr, Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use std::collections::HashMap;
use uuid::Uuid;
use std::error::Error;
use std::time::Duration;
use tokio::time;

const COMBUSTION_VENDOR_ID: u16 = 0x09C7;
const SCAN_DURATION: u64 = 10;

#[derive(Debug)]
struct ManufacturerData {
    vendor_id: Option<u16>,
}

#[derive(Debug)]
struct PeripheralProperties {
    address: BDAddr,
    address_type: Option<AddressType>,
    local_name: Option<String>,
    tx_power_level: i16,
    rssi: i16,
    manufacturer_data: ManufacturerData,
    service_data: HashMap<Uuid, Vec<u8>>,
    services: Vec<Uuid>,
    class: Option<u32>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Combustion Inc. BLE Predictive Probe Example");

  // Initialize Bluetooth manager
  let manager = Manager::new().await?;

  // Get adapters
  let adapters = manager.adapters().await?;

  // Get the first adapter or print error
  let central = adapters.into_iter().nth(0).ok_or("No Bluetooth adapters found")?;

  println!("Scanning for Combustion Inc. Predictive Probe devices...");

  // Start scanning
  central.start_scan(ScanFilter::default()).await?;

  // Wait a bit for devices to be discovered
  time::sleep(Duration::from_secs(SCAN_DURATION)).await;

  // Get discovered peripherals
  let peripherals = central.peripherals().await?;
  
  // Check if any devices were found
if peripherals.is_empty() {
    println!("No BLE devices found.");
    return Ok(());
  }

  // Iterate through discovered peripherals
  for peripheral in peripherals {

    //Print all peripheral properties
    let peripheral_properties = get_peripheral_properties(&peripheral).await?;

    if peripheral_properties.manufacturer_data.vendor_id.is_some() {
      println!("{:#?}", peripheral_properties);

      println!("Connecting...");

      peripheral.connect().await?;
      println!("Connected");
      
      // Keep connection alive for a bit to verify
      time::sleep(Duration::from_secs(5)).await;
      
      println!("Disconnecting...");
      peripheral.disconnect().await?;
      
      return Ok(());
    }
  }

  println!("No Combustion Inc. devices found.");
  Ok(())
}

//Get manufacturer data from the manufacturer data hash map
async fn get_manufacturer_data(manufacturer_data_hashmap: HashMap<u16, Vec<u8>>) -> Result<ManufacturerData, Box<dyn Error>> {
  let vendor_id = if manufacturer_data_hashmap.contains_key(&COMBUSTION_VENDOR_ID) {
    Some(COMBUSTION_VENDOR_ID)
  } else {
    None
  };

  Ok(ManufacturerData { vendor_id })
}

//Get peripheral properties from the peripheral
async fn get_peripheral_properties<P: Peripheral>(peripheral: &P) -> Result<PeripheralProperties, Box<dyn Error>> {
  let properties = peripheral.properties().await?.unwrap();
  let manufacturer_data_hashmap = properties.manufacturer_data;

  let manufacturer_data = get_manufacturer_data(manufacturer_data_hashmap.clone()).await?;

  //Unwrap each property
  let address = properties.address;
  let address_type = properties.address_type;
  let local_name = properties.local_name;
  let tx_power_level = properties.tx_power_level.unwrap_or_default();
  let rssi = properties.rssi.unwrap_or_default();
  let service_data = properties.service_data;
  let services = properties.services;
  let class = properties.class;

  Ok(PeripheralProperties {
      address,
      address_type,
      local_name,
      tx_power_level,
      rssi,
      manufacturer_data,
      service_data,
      services,
      class,
  })
}
  