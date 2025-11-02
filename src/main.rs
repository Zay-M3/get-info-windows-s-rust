
use sysinfo::{System};

use crate::app::start_menu_app;

mod app;
mod functions;
mod utils;
mod api;
mod scanner;


#[tokio::main]
async fn main() {
    // Create a single System instance and refresh all data once
    let mut sys = System::new_all();
    sys.refresh_all();

    start_menu_app(&mut sys).await;
    println!("Press Enter to exit...");
    
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

