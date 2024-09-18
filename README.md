# RustLogger: <br/>
 This repository contains two components: a RustLogger Client and a RustLogger Server. The client captures keystrokes and transmits them to the server, which logs them for analysis.

### Prerequisites
- Rust: Ensure you have Rust installed on your system.
- Cargo: Comes bundled with Rust for building and managing dependencies.

RustLogger-client/: Contains the client application.
RustLogger-server/: Contains the server application.

### Instructions <br/>
Build & Run Client
Navigate to the client directory:
```
cd RustLogger/RustLogger-client
```
Build the client using Cargo:
```
cargo build --release
```
Run the client:
```
cargo run
```
Build & Run Server
Navigate to the server directory:
```
cd RustLogger/RustLogger-server
```
Build the server:
```
cargo build --release
```
Run the server:
```
cargo run
```
### Logging Keystrokes:
- The keylogger server will store captured keystrokes in keystrokes.txt.

### Contributions
- Feel free to fork this repository and contribute!
