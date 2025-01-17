#![windows_subsystem = "windows"]
use app_componts::{App,Setting,initialize_application};
use tokio::time::Duration;


#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let app_window = App::new()?;
    let setting_window = Setting::new()?;
    initialize_application(
        app_window,
        setting_window,
        1000,  // max_logs
        100,   // batch_size
        Duration::from_millis(100)  // update_interval
    ).await;
    Ok(())

}
