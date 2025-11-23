# Planned GitHub Issues

## Issue 1: Project Setup & BLE Scanner
**Title:** Setup Project and Implement BLE Scanner
**Description:**
- Add necessary dependencies to `Cargo.toml`: `btleplug`, `tokio`, `uuid`, `futures`.
- Implement a BLE scanner in `main.rs` that filters for devices with Vendor ID `0x09C7` (Combustion Inc.).
- Implement basic connection logic to the first discovered device.
- Verify connection by printing "Connected to [Device Name]".

## Issue 2: Device Information Service
**Title:** Implement Device Information Service Reading
**Description:**
- Create `src/device_info.rs`.
- Implement functions to read characteristics from the Device Information Service (`0x180A`).
- Read and print:
    - Manufacturer Name
    - Model Number
    - Serial Number
    - Firmware Revision
    - Hardware Revision

## Issue 3: Real-time Data Parsing
**Title:** Implement Real-time Probe Status Parsing
**Description:**
- Create `src/protocol.rs`.
- Implement data structures and parsers for the Probe Status characteristic (`00000101-CAAB-3792-3D44-97AE51C1407A`).
- Handle bit-packed fields:
    - Raw Temperature (13 bytes)
    - Mode/ID
    - Battery Status
    - Virtual Sensors
    - Prediction Status
- Update `main.rs` to subscribe to notifications and print parsed data.

## Issue 4: UART & Control
**Title:** Implement UART Service and Control
**Description:**
- Create `src/uart.rs`.
- Implement the Nordic UART Service (`6E400001-B5A3-F393-E0A9-E50E24DCCA9E`).
- Implement sending commands (e.g., Set Color, Set Prediction).
- Implement receiving responses.
- Add a basic CLI loop to `main.rs` to trigger commands.
