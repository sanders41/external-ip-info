use std::{
    fs::{create_dir_all, read_to_string, File},
    path::PathBuf,
};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
    pub ip_addr: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            ip_addr: None,
            country: None,
            region: None,
            city: None,
        }
    }

    pub fn clear_cache() -> Result<()> {
        let cache = Self::new();
        cache.save()?;

        Ok(())
    }

    pub fn get() -> Result<Self> {
        if let Some(cache_file) = cache_file_path() {
            if cache_file.exists() {
                if let Ok(cache_str) = read_to_string(cache_file) {
                    if let Ok(cache) = serde_json::from_str(&cache_str) {
                        return Ok(cache);
                    }
                }
            }
        }

        Ok(Self::new())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(c) = cache_dir() {
            if !c.exists() {
                create_dir_all(c)?;
            }

            if let Some(cp) = cache_file_path() {
                let cache_file = File::create(cp)?;
                serde_json::to_writer(cache_file, &self)?;
            } else {
                bail!("Unable to save cache file.")
            }
        } else {
            bail!("Unable to save cache file.");
        }

        Ok(())
    }
}

fn cache_dir() -> Option<PathBuf> {
    let config_dir = dirs::cache_dir();

    if let Some(mut c) = config_dir {
        c.push("external-ip-info");
        Some(c)
    } else {
        None
    }
}

fn cache_file_path() -> Option<PathBuf> {
    if let Some(mut c) = cache_dir() {
        c.push("cache.json");
        Some(c)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_dir_exists() {
        let cache_dir = cache_dir();
        assert_ne!(cache_dir, None);

        if let Some(c) = cache_dir {
            let last = c.file_name();
            assert_ne!(last, None);
            if let Some(l) = last {
                assert_eq!(l, "external-ip-info");
            }
        }
    }

    #[test]
    fn cache_path_exists() {
        let cache_file_path = cache_file_path();
        assert_ne!(cache_file_path, None);

        if let Some(mut c) = cache_file_path {
            let last = c.file_name();
            assert_ne!(last, None);
            if let Some(l) = last {
                assert_eq!(l, "cache.json");
            }

            c.pop();
            let dir = c.file_name();
            assert_ne!(dir, None);
            if let Some(d) = dir {
                assert_eq!(d, "external-ip-info");
            }
        }
    }
}
