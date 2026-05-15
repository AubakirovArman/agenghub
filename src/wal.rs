use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Wal {
    tx_id: String,
    path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalRecord {
    pub sequence: u64,
    pub ts: DateTime<Utc>,
    pub tx_id: String,
    pub state: String,
    pub message: String,
    #[serde(default)]
    pub data: Value,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalReplay {
    pub record_count: usize,
    pub latest_state: Option<String>,
    pub records: Vec<WalRecord>,
}

#[derive(Debug, Serialize)]
struct ChecksumPayload<'a> {
    sequence: u64,
    ts: DateTime<Utc>,
    tx_id: &'a str,
    state: &'a str,
    message: &'a str,
    data: &'a Value,
}

impl Wal {
    pub fn new(tx_id: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            tx_id: tx_id.into(),
            path: path.into(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(&self, state: &str, message: &str, data: &Value) -> Result<WalRecord> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        let sequence = next_sequence(&self.path)?;
        let ts = Utc::now();
        let checksum = checksum(sequence, ts, &self.tx_id, state, message, data)?;
        let record = WalRecord {
            sequence,
            ts,
            tx_id: self.tx_id.clone(),
            state: state.to_string(),
            message: message.to_string(),
            data: data.clone(),
            checksum,
        };
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .with_context(|| format!("open {}", self.path.display()))?;
        writeln!(file, "{}", serde_json::to_string(&record)?)
            .with_context(|| format!("append {}", self.path.display()))?;
        file.flush()?;
        file.sync_data()?;
        Ok(record)
    }
}

pub fn replay(path: &Path) -> Result<WalReplay> {
    if !path.exists() {
        return Ok(WalReplay {
            record_count: 0,
            latest_state: None,
            records: Vec::new(),
        });
    }
    let file = fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut records = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let record: WalRecord =
            serde_json::from_str(&line).with_context(|| format!("parse {}", path.display()))?;
        let expected = records.len() as u64 + 1;
        if record.sequence != expected {
            return Err(anyhow!(
                "WAL sequence mismatch at line {}; expected {}, got {}",
                index + 1,
                expected,
                record.sequence
            ));
        }
        verify_checksum(&record)?;
        records.push(record);
    }
    let latest_state = records.last().map(|record| record.state.clone());
    Ok(WalReplay {
        record_count: records.len(),
        latest_state,
        records,
    })
}

pub fn write_replay(path: &Path, output: &Path) -> Result<WalReplay> {
    let replay = replay(path)?;
    fs::write(output, serde_json::to_string_pretty(&replay)?)
        .with_context(|| format!("write {}", output.display()))?;
    Ok(replay)
}

fn next_sequence(path: &Path) -> Result<u64> {
    if !path.exists() {
        return Ok(1);
    }
    let file = fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let count = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .count();
    Ok(count as u64 + 1)
}

fn verify_checksum(record: &WalRecord) -> Result<()> {
    let computed = checksum(
        record.sequence,
        record.ts,
        &record.tx_id,
        &record.state,
        &record.message,
        &record.data,
    )?;
    if computed != record.checksum {
        return Err(anyhow!(
            "WAL checksum mismatch at sequence {}",
            record.sequence
        ));
    }
    Ok(())
}

fn checksum(
    sequence: u64,
    ts: DateTime<Utc>,
    tx_id: &str,
    state: &str,
    message: &str,
    data: &Value,
) -> Result<String> {
    let payload = ChecksumPayload {
        sequence,
        ts,
        tx_id,
        state,
        message,
        data,
    };
    let mut hasher = Sha256::new();
    hasher.update(serde_json::to_vec(&payload)?);
    Ok(hex_lower(&hasher.finalize()))
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests;
