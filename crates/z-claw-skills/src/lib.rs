use serde::{Deserialize, Serialize};
use std::path::Path;

/// Frontmatter parsed from a SKILL.md file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub paths: Vec<String>,
}

/// A loaded skill with frontmatter + body content.
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub paths: Vec<String>,
    pub body: String,
    pub source: SkillSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillSource {
    User,
    Project,
    Bundled,
}

impl Skill {
    /// Parse a SKILL.md string into a Skill.
    pub fn from_markdown(content: &str, source: SkillSource) -> Option<Self> {
        let (frontmatter_str, body) = extract_frontmatter(content)?;
        let fm: SkillFrontmatter = serde_yaml::from_str(&frontmatter_str).ok()?;
        Some(Skill {
            name: fm.name,
            description: fm.description,
            paths: fm.paths,
            body: body.trim().to_string(),
            source,
        })
    }

    /// Check if this skill activates for the given file path.
    pub fn matches_path(&self, file_path: &str) -> bool {
        if self.paths.is_empty() {
            return false;
        }
        self.paths
            .iter()
            .any(|pattern| glob_match::glob_match(pattern, file_path))
    }

    /// Format the skill for system prompt injection.
    pub fn to_prompt_section(&self) -> String {
        format!(
            "## Skill: {}\n{}\n\n{}",
            self.name, self.description, self.body
        )
    }
}

/// Registry of loaded skills with path-based lookup.
pub struct SkillRegistry {
    skills: Vec<Skill>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn register(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    /// Find skills matching the given file path.
    pub fn find_by_path(&self, file_path: &str) -> Vec<&Skill> {
        self.skills
            .iter()
            .filter(|s| s.matches_path(file_path))
            .collect()
    }

    pub fn list(&self) -> &[Skill] {
        &self.skills
    }

    /// Load all .md files from a directory as skills.
    pub fn load_dir(&mut self, dir: &Path, source: SkillSource) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let is_md = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.ends_with(".md"))
                    .unwrap_or(false);
                if !is_md {
                    continue;
                }
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Some(skill) = Skill::from_markdown(&content, source.clone()) {
                        tracing::info!("Loaded skill: {}", skill.name);
                        self.register(skill);
                    }
                }
            }
        }
    }

    /// Get active skills as a prompt section for the given file path.
    pub fn active_skills_prompt(&self, file_path: Option<&str>) -> String {
        let skills: Vec<&Skill> = if let Some(fp) = file_path {
            self.find_by_path(fp)
        } else {
            self.skills.iter().collect()
        };
        if skills.is_empty() {
            return String::new();
        }
        format!(
            "## Available Skills\n\n{}",
            skills
                .iter()
                .map(|s| s.to_prompt_section())
                .collect::<Vec<_>>()
                .join("\n\n---\n\n")
        )
    }
}

fn extract_frontmatter(content: &str) -> Option<(String, String)> {
    let content = content.trim();
    if !content.starts_with("---") {
        return None;
    }
    let after = &content[3..];
    let end = after.find("---")?;
    let fm = after[..end].trim().to_string();
    let body = after[end + 3..].to_string();
    Some((fm, body))
}
