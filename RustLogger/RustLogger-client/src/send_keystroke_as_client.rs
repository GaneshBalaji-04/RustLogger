use reqwest::blocking::Client;

pub fn send_keystroke_to_server(keystroke:&String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.post("http://127.0.0.1:3030/keystrokes")
        .body(keystroke.to_string())
        .send()?;
    
    if res.status().is_success() {
        Ok(())
    }
    else {
        Err(format!("Failed to send the keystroke. Status: {}",res.status()).into())
    }
}