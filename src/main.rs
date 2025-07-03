mod cache;
mod cli;

use anyhow::Result;
use clap::Parser;
use colored::*;
use serde::Deserialize;

use crate::cache::Cache;
use crate::cli::Cli;

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
    let args = Cli::parse();

    if args.clear_cache && Cache::clear_cache().is_err() {
        eprintln!("{}", "Error clearing cache".red());
        std::process::exit(1);
    }

    let ip_info = if let Ok(ip) = get_ip_info() {
        ip
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };

    let cache = if let Ok(mut c) = Cache::get() {
        if let Some(ip_addr) = &c.ip_addr {
            if ip_addr == &ip_info.ip_addr {
                c
            } else if let Ok(location) = get_location_info(&ip_info.ip_addr) {
                c.ip_addr = Some(ip_info.ip_addr);
                c.country = Some(location.country);
                c.region = Some(location.region);
                c.city = Some(location.city);

                if c.save().is_err() {
                    eprintln!("{}", "Error saving cache".red());
                    std::process::exit(1);
                }

                c
            } else {
                eprintln!("{}", "Error retrieving location information".red());
                std::process::exit(1);
            }
        } else if let Ok(location) = get_location_info(&ip_info.ip_addr) {
            c.ip_addr = Some(ip_info.ip_addr);
            c.country = Some(location.country);
            c.region = Some(location.region);
            c.city = Some(location.city);

            if c.save().is_err() {
                eprintln!("{}", "Error saving cache".red());
                std::process::exit(1);
            }

            c
        } else {
            eprintln!("{}", "Error retrieving location information".red());
            std::process::exit(1);
        }
    } else {
        eprintln!("{}", "Error retrieving cache".red());
        std::process::exit(1);
    };

    let ip_addr = if let Some(ip_addr) = cache.ip_addr {
        format!("{}", ip_addr.blue())
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };
    let country = if let Some(country) = cache.country {
        format!("{}", country.blue())
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };
    let region = if let Some(region) = cache.region {
        format!("{}", region.blue())
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };
    let city = if let Some(city) = cache.city {
        format!("{}", city.blue())
    } else {
        eprintln!("{}", "Error retrieving ip information".red());
        std::process::exit(1);
    };

    println!();
    println!("ip address: {ip_addr}");
    println!("country: {country}");
    println!("region: {region}");
    println!("city: {city}");
}
