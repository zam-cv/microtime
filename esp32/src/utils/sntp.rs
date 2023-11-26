use anyhow::Result;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};

pub fn init() -> Result<()> {
    let sntp = EspSntp::new_default()?;
    log::info!("SNTP initialized, waiting for status!");

    while sntp.get_sync_status() != SyncStatus::Completed {
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    log::info!("SNTP status received!");

    Ok(())
}