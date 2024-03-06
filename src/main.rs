use anyhow::Result;
use colored::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IpInfo {
    ip_addr: String,
}

#[derive(Debug, Deserialize)]
struct LocationInfo {
    country: String,
    region: String,
    city: String,
}

fn get_ip_info() -> Result<IpInfo> {
    let ip = reqwest::blocking::get("https://ifconfig.me/all.json")?.json::<IpInfo>()?;

    Ok(ip)
}

fn get_location_info(ip: &str) -> Result<LocationInfo> {
    let location =
        reqwest::blocking::get(format!("http://ip-api.com/json/{ip}"))?.json::<LocationInfo>()?;

    Ok(location)
}

fn main() {
    let ip_info = if let Ok(ip) = get_ip_info() {
        ip
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };
    let location_info = if let Ok(location) = get_location_info(&ip_info.ip_addr) {
        location
    } else {
        eprintln!("{}", "Error retrieving location information".red());
        std::process::exit(1);
    };

    let ip_addr = format!("{}", ip_info.ip_addr.blue());
    let country = format!("{}", location_info.country.blue());
    let region = format!("{}", location_info.region.blue());
    let city = format!("{}", location_info.city.blue());

    println!();
    println!("ip address: {}", ip_addr);
    println!("country: {}", country);
    println!("region: {}", region);
    println!("city: {}", city);
}
