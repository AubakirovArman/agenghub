use crate::domain_runtime::RuntimePack;

pub fn packs() -> Vec<RuntimePack> {
    vec![
        pack(PackSpec {
            id: "code.rust",
            domain: "code",
            name: "Rust Package",
            workspaces: &["code.git"],
            verifiers: &["code_build"],
            effects: &["file", "dependency", "artifact"],
            artifacts: &["Cargo.toml", "target/", "domain_runtime.json"],
            memory: &["code.memory.v1"],
            tools: &["cargo"],
        }),
        pack(PackSpec {
            id: "infra.terraform",
            domain: "infra",
            name: "Terraform Plan",
            workspaces: &["infra.git"],
            verifiers: &["infra_plan"],
            effects: &["file", "terraform_state", "cloud"],
            artifacts: &["infra/**/*.tf", "terraform.plan", "domain_runtime.json"],
            memory: &["infra.memory.v1"],
            tools: &["terraform"],
        }),
        pack(PackSpec {
            id: "data.python",
            domain: "data",
            name: "Python Data Runtime",
            workspaces: &["data.git"],
            verifiers: &["data_quality"],
            effects: &["file", "artifact", "dependency"],
            artifacts: &["data/**/*.json", "notebooks/", "domain_runtime.json"],
            memory: &["data.memory.v1"],
            tools: &["python"],
        }),
        pack(PackSpec {
            id: "media.render",
            domain: "media",
            name: "Media Render Runtime",
            workspaces: &["media.git"],
            verifiers: &["media_render"],
            effects: &["file", "artifact", "process"],
            artifacts: &["media/**/*", "renders/", "domain_runtime.json"],
            memory: &["content.memory.v1"],
            tools: &["ffmpeg"],
        }),
        pack(PackSpec {
            id: "research.citations",
            domain: "research",
            name: "Research Citation Runtime",
            workspaces: &["research.git"],
            verifiers: &["research_report"],
            effects: &["file", "network", "artifact"],
            artifacts: &[
                "research/report.md",
                "research/sources.json",
                "domain_runtime.json",
            ],
            memory: &["content.memory.v1"],
            tools: &[],
        }),
    ]
}

struct PackSpec<'a> {
    id: &'a str,
    domain: &'a str,
    name: &'a str,
    workspaces: &'a [&'a str],
    verifiers: &'a [&'a str],
    effects: &'a [&'a str],
    artifacts: &'a [&'a str],
    memory: &'a [&'a str],
    tools: &'a [&'a str],
}

fn pack(spec: PackSpec<'_>) -> RuntimePack {
    RuntimePack {
        id: spec.id.to_string(),
        domain: spec.domain.to_string(),
        name: spec.name.to_string(),
        supported_workspaces: strings(spec.workspaces),
        verifier_profiles: strings(spec.verifiers),
        effects: strings(spec.effects),
        artifacts: strings(spec.artifacts),
        memory_schemas: strings(spec.memory),
        required_tools: strings(spec.tools),
        warnings: Vec::new(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
