use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BuildResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version: Option<String>,
    pub build: Option<i64>,
    pub time: Option<String>,
    pub channel: Option<String>,
    pub promoted: Option<bool>,
    pub changes: Option<Vec<Change>>,
    pub downloads: Option<serde_json::Value>,
}
impl std::fmt::Display for BuildResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BuildsResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version: Option<String>,
    pub builds: Option<Vec<VersionBuild>>,
}
impl std::fmt::Display for BuildsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Change {
    pub commit: Option<String>,
    pub summary: Option<String>,
    pub message: Option<String>,
}
impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Download {
    pub name: Option<String>,
    pub sha256: Option<String>,
}
impl std::fmt::Display for Download {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version_groups: Option<Vec<String>>,
    pub versions: Option<Vec<String>>,
}
impl std::fmt::Display for ProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsResponse {
    pub projects: Option<Vec<String>>,
}
impl std::fmt::Display for ProjectsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionBuild {
    pub build: Option<i64>,
    pub time: Option<String>,
    pub channel: Option<String>,
    pub promoted: Option<bool>,
    pub changes: Option<Vec<Change>>,
    pub downloads: Option<serde_json::Value>,
}
impl std::fmt::Display for VersionBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionFamilyBuild {
    pub version: Option<String>,
    pub build: Option<i64>,
    pub time: Option<String>,
    pub channel: Option<String>,
    pub promoted: Option<bool>,
    pub changes: Option<Vec<Change>>,
    pub downloads: Option<serde_json::Value>,
}
impl std::fmt::Display for VersionFamilyBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionFamilyBuildsResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version_group: Option<String>,
    pub versions: Option<Vec<String>>,
    pub builds: Option<Vec<VersionFamilyBuild>>,
}
impl std::fmt::Display for VersionFamilyBuildsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionFamilyResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version_group: Option<String>,
    pub versions: Option<Vec<String>>,
}
impl std::fmt::Display for VersionFamilyResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersionResponse {
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version: Option<String>,
    pub builds: Option<Vec<i64>>,
}
impl std::fmt::Display for VersionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
