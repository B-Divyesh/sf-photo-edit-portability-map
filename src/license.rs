//! Sociobot one-time license verification and a daily local verdict cache.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SLUG: &str = "photo-edit-portability-map";
const VERIFY_URL: &str =
    "https://api.sociobot.in/api/v1/products/photo-edit-portability-map/verify";
const TOKEN_ENV: &str = "EDIT_PORTABILITY_MAP_LICENSE";
const DAY_SECONDS: u64 = 86_400;

#[derive(Debug, Deserialize)]
struct VerifyResponse {
    valid: bool,
    reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LicenseCache {
    token: String,
    valid: bool,
    checked_at: u64,
    attempted_at: u64,
}

pub fn activate(token: &str) -> Result<()> {
    let token = token.trim();
    if token.is_empty() {
        bail!("license token cannot be empty");
    }
    let now = now()?;
    let response = request_verdict(token).context("license verification failed")?;
    write_cache(&LicenseCache {
        token: token.to_owned(),
        valid: response.valid,
        checked_at: now,
        attempted_at: now,
    })?;
    if response.valid {
        Ok(())
    } else {
        bail!("license is not active ({})", response.reason)
    }
}

pub fn status() -> Result<String> {
    match read_cache()? {
        Some(cache) if cache.valid => Ok(format!(
            "Pro license cached as active; last verified at Unix time {}.",
            cache.checked_at
        )),
        Some(_) => Ok("A license is stored but is not active.".to_owned()),
        None => Ok("No Pro license is stored. The free edition is active.".to_owned()),
    }
}

pub fn require_pro() -> Result<()> {
    let now = now()?;
    let cached = read_cache()?;
    let token_from_env = std::env::var(TOKEN_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty());
    let token = token_from_env
        .as_deref()
        .or_else(|| cached.as_ref().map(|value| value.token.as_str()))
        .context("a Pro license is required for samples above 10; run `edit-portability-map license activate <TOKEN>` or set EDIT_PORTABILITY_MAP_LICENSE")?;

    let same_cached_token = cached.as_ref().filter(|cache| cache.token == token);
    if let Some(cache) = same_cached_token {
        if now.saturating_sub(cache.attempted_at) < DAY_SECONDS {
            return if cache.valid {
                Ok(())
            } else {
                bail!("the stored Pro license is not active")
            };
        }
    }

    let previous_valid = same_cached_token.is_some_and(|cache| cache.valid);
    let attempt_cache = LicenseCache {
        token: token.to_owned(),
        valid: previous_valid,
        checked_at: same_cached_token.map_or(0, |cache| cache.checked_at),
        attempted_at: now,
    };
    write_cache(&attempt_cache)?;
    match request_verdict(token) {
        Ok(response) => {
            write_cache(&LicenseCache {
                token: token.to_owned(),
                valid: response.valid,
                checked_at: now,
                attempted_at: now,
            })?;
            if response.valid {
                Ok(())
            } else {
                bail!("the Pro license is not active ({})", response.reason)
            }
        }
        Err(error) if previous_valid => {
            eprintln!("warning: license recheck postponed ({error}); using the last valid verdict");
            Ok(())
        }
        Err(error) => {
            Err(error).context("could not verify the Pro license; reconnect and try again")
        }
    }
}

fn request_verdict(token: &str) -> Result<VerifyResponse> {
    let response = ureq::get(VERIFY_URL)
        .query("license", token)
        .set("Accept", "application/json")
        .timeout(std::time::Duration::from_secs(12))
        .call()
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    response
        .into_json::<VerifyResponse>()
        .context("license server returned an unreadable response")
}

fn cache_path() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join(SLUG).join("license.json"));
    }
    if cfg!(windows) {
        if let Some(path) = std::env::var_os("APPDATA") {
            return Ok(PathBuf::from(path).join(SLUG).join("license.json"));
        }
    }
    let home = std::env::var_os("HOME").context("cannot locate a user config directory")?;
    Ok(PathBuf::from(home)
        .join(".config")
        .join(SLUG)
        .join("license.json"))
}

fn read_cache() -> Result<Option<LicenseCache>> {
    let path = cache_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("could not read license cache {}", path.display()))?;
    let cache = serde_json::from_str(&text)
        .with_context(|| format!("license cache is invalid: {}", path.display()))?;
    Ok(Some(cache))
}

fn write_cache(cache: &LicenseCache) -> Result<()> {
    let path = cache_path()?;
    let parent = path.parent().context("invalid license cache path")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("could not create config folder {}", parent.display()))?;
    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(&path)
        .with_context(|| format!("could not write license cache {}", path.display()))?;
    file.write_all(serde_json::to_string(cache)?.as_bytes())?;
    Ok(())
}

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
