use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PluginCommands {
    List,
    Inspect {
        package: PathBuf,
    },
    Digest {
        package: PathBuf,
    },
    Scaffold {
        output: PathBuf,

        #[arg(long)]
        package_id: String,

        #[arg(long)]
        skill_id: String,

        #[arg(long)]
        description: String,

        #[arg(long)]
        author: Option<String>,

        #[arg(long)]
        force: bool,
    },
    Install {
        package: PathBuf,

        #[arg(long, default_value = "local")]
        trust: String,

        #[arg(long)]
        allow_untrusted: bool,

        #[arg(long)]
        force: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum EnterpriseCommands {
    Policy,
    PolicyServer {
        #[arg(long, default_value = "127.0.0.1:8787")]
        bind: String,

        #[arg(long)]
        policy: Option<PathBuf>,

        #[arg(long, default_value = "AGENTHUB_POLICY_TOKEN")]
        token_env: String,

        #[arg(long)]
        once: bool,
    },
    Secrets {
        name: Option<String>,
    },
    Runners,
    ModelRoute {
        model: String,
    },
    Audit {
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
    Compliance {
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    List,
}
