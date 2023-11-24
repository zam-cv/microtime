use anyhow::Result;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};

pub fn init() -> Result<()> {
    let sntp = EspSntp::new_default()?;
    log::info!("SNTP initialized, waiting for status!");

    while sntp.get_sync_status() != SyncStatus::Completed {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    std::env::set_var("TZ", "CST6CDT,M4.1.0,M10.5.0");
    log::info!("SNTP status received!");

    Ok(())
}
