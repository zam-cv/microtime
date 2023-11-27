use anyhow::Result;
use esp_idf_svc::sntp::{EspSntp, SyncStatus};
use std::{thread, time::Duration};

pub fn init() -> Result<()> {
    let sntp = EspSntp::new_default()?;
    log::info!("SNTP initialized, waiting for status!");

    while sntp.get_sync_status() != SyncStatus::Completed {
        thread::sleep(Duration::from_millis(200));
    }

    log::info!("SNTP status received!");

    Ok(())
}