#[cfg(feature = "nethost")]
mod nethost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "nethost")]
    nethost_build::download_and_link_nethost()?;

    Ok(())
}
