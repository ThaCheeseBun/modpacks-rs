use serde::Deserialize;

#[derive(Deserialize)]
pub struct VersionManifest {
    pub files: Vec<VersionManifestFile>,
    pub targets: Vec<VersionManifestTarget>,
    pub parent: u64,
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct VersionManifestFile {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: u64,
    pub clientonly: bool,
    pub serveronly: bool,
    pub optional: bool,
    pub name: String,
    pub curseforge: Option<VersionManifestFileCurseForge>,
}

#[derive(Deserialize)]
pub struct VersionManifestFileCurseForge {
    pub project: u64,
    pub file: u64,
}

#[derive(Deserialize)]
pub struct VersionManifestTarget {
    pub version: String,
    pub name: String,
}
