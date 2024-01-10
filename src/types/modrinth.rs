use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub format_version: u64,
    pub game: String,
    pub version_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub files: Vec<FormatFile>,
    pub dependencies: FormatDeps,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatFile {
    pub path: String,
    pub hashes: FormatFileHashes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<FormatFileEnv>,
    pub downloads: Vec<String>,
    pub file_size: u64,
}

#[derive(Serialize)]
pub struct FormatFileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct FormatDeps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minecraft: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neoforge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabric_loader: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quilt_loader: Option<String>,
}

/*
    required
    optional
    unsupported
*/
#[derive(Serialize)]
pub struct FormatFileEnv {
    pub client: String,
    pub server: String,
}
