use anyhow::Result;

use super::defaults::{
    DEFAULT_CONTENT_SCHEMA, DEFAULT_DATA_SCHEMA, DEFAULT_INFRA_SCHEMA, DEFAULT_MEDIA_SCHEMA,
    DEFAULT_RESEARCH_SCHEMA,
};
use super::{write_default, AgentPaths};

pub(super) fn write_defaults(paths: &AgentPaths, force: bool) -> Result<()> {
    for (name, content) in [
        ("content.yaml", DEFAULT_CONTENT_SCHEMA),
        ("data.yaml", DEFAULT_DATA_SCHEMA),
        ("infra.yaml", DEFAULT_INFRA_SCHEMA),
        ("media.yaml", DEFAULT_MEDIA_SCHEMA),
        ("research.yaml", DEFAULT_RESEARCH_SCHEMA),
    ] {
        write_default(&paths.schemas.join(name), content, force)?;
    }
    Ok(())
}
