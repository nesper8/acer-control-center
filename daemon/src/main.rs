mod backend;
mod dbus;
mod ffi;
mod service;
mod sysfs;

use std::sync::Arc;

use anyhow::Result;
use dbus::AcerControlDBus;
use service::AcerService;
use sysfs::SysfsBackend;
use zbus::connection::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    let backend = Arc::new(SysfsBackend::new());

    let service = Arc::new(AcerService::new(backend));

    let _connection = Builder::system()?
        .name("io.github.stanlysilas.AcerControl")?
        .serve_at(
            "/io/github/stanlysilas/AcerControl",
            AcerControlDBus::new(service),
        )?
        .build()
        .await?;

    println!("Acer Control Daemon started.");

    std::future::pending::<()>().await;

    Ok(())
}
