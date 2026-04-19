use mdns_sd::{ServiceDaemon, ServiceInfo};

const SERVICE_TYPE: &str = "_iris._tcp.local.";
const INSTANCE_NAME: &str = "iris";
const HOSTNAME: &str = "iris.local.";

pub fn create_multicast_advertiser(port: u16) -> Result<ServiceDaemon, Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;

    let iris_service = ServiceInfo::new(
        SERVICE_TYPE,
        INSTANCE_NAME,
        HOSTNAME,
        "",
        port,
        &[("txtvers", "1"), ("version", "0.0.1")][..],
    )?
    .enable_addr_auto();

    mdns.register(iris_service)?;

    Ok(mdns)
}
