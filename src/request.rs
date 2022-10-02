use serde_json::json;
use crate::model::*;
use crate::PaperMcClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProjectsRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
}
impl<'a> ProjectsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectsResponse> {
        let mut r = self.client.client.get("/v2/projects");
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProjectRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
}
impl<'a> ProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectResponse> {
        let mut r = self
            .client
            .client
            .get(&format!("/v2/projects/{project}", project = self.project));
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct VersionRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub version: String,
}
impl<'a> VersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VersionResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}", project = self.project,
                    version = self.version
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BuildsRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub version: String,
}
impl<'a> BuildsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BuildsResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}/builds", project = self
                    .project, version = self.version
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BuildRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub version: String,
    pub build: i64,
}
impl<'a> BuildRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BuildResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}/builds/{build}", project =
                    self.project, version = self.version, build = self.build
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DownloadRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub version: String,
    pub build: i64,
    pub download: String,
}
impl<'a> DownloadRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/versions/{version}/builds/{build}/downloads/{download}",
                    project = self.project, version = self.version, build = self.build,
                    download = self.download
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DownloadRequired<'a> {
    pub project: &'a str,
    pub version: &'a str,
    pub build: i64,
    pub download: &'a str,
}
impl<'a> DownloadRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct FamilyRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub family: String,
}
impl<'a> FamilyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VersionFamilyResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/version_group/{family}", project = self
                    .project, family = self.family
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct FamilyBuildsRequest<'a> {
    pub(crate) client: &'a PaperMcClient,
    pub project: String,
    pub family: String,
}
impl<'a> FamilyBuildsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<VersionFamilyBuildsResponse> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v2/projects/{project}/version_group/{family}/builds", project =
                    self.project, family = self.family
                ),
            );
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
