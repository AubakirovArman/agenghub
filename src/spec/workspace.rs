use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSpec {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub isolation: Option<String>,
    #[serde(default)]
    pub root: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceProfile {
    Code,
    Content,
    Data,
    Infra,
    Media,
    Research,
}

impl WorkspaceSpec {
    pub fn profile(&self) -> Result<WorkspaceProfile> {
        match self.kind.as_str() {
            "code.git" => Ok(WorkspaceProfile::Code),
            "content.git" => Ok(WorkspaceProfile::Content),
            "data.git" => Ok(WorkspaceProfile::Data),
            "infra.git" => Ok(WorkspaceProfile::Infra),
            "media.git" => Ok(WorkspaceProfile::Media),
            "research.git" => Ok(WorkspaceProfile::Research),
            other => Err(anyhow!(
                "unsupported workspace.type `{other}`; supported: code.git, content.git, data.git, infra.git, media.git, research.git"
            )),
        }
    }
}

impl WorkspaceProfile {
    pub fn domain(self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Content => "content",
            Self::Data => "data",
            Self::Infra => "infra",
            Self::Media => "media",
            Self::Research => "research",
        }
    }

    pub fn memory_change_kind(self) -> &'static str {
        match self {
            Self::Code => "code_change",
            Self::Content => "content_change",
            Self::Data => "data_change",
            Self::Infra => "infra_change",
            Self::Media => "media_change",
            Self::Research => "research_change",
        }
    }
}
