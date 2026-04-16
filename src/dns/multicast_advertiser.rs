use mdns_sd::{ServiceDaemon, ServiceInfo};

const SERVICE_TYPE: &str = "_iris._tcp.local.";
const INSTANCE_NAME: &str = "iris";
const IP_ADDRESS: &str = "192.168.1.1";
const HOSTNAME: &str = "iris";
const PORT: u16 = 8080;
const PROPERTIES: [(&str, &str); 5] = [
    ("path", "/"),
    ("port", "8080"),
    ("txtvers", "1"),
    ("txt", "name=iris"),
    ("version", "0.0.1"),
];
pub fn create_multicast_advertiser() -> Result<ServiceDaemon, Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;

    let iris_service = ServiceInfo::new(
        SERVICE_TYPE,
        INSTANCE_NAME,
        HOSTNAME,
        IP_ADDRESS,
        PORT,
        &PROPERTIES[..],
    )?;

    mdns.register(iris_service)?;

    Ok(mdns)
}
