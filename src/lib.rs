//! [`PaperMcClient`](struct.PaperMcClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct PaperMcClient {
    pub(crate) client: httpclient::Client,
}
impl PaperMcClient {
    pub fn from_env() -> Self {
        let url = std::env::var("PAPER_MC_BASE_URL")
            .expect("Missing environment variable PAPER_MC_BASE_URL");
        Self::new(&url)
    }
}
impl PaperMcClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        PaperMcClient { client }
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///Gets a list of all available projects.
    pub fn projects(&self) -> request::ProjectsRequest {
        request::ProjectsRequest {
            client: &self,
        }
    }
    ///Gets information about a project.
    pub fn project(&self, project: &str) -> request::ProjectRequest {
        request::ProjectRequest {
            client: &self,
            project: project.to_owned(),
        }
    }
    ///Gets information about a version.
    pub fn version(&self, project: &str, version: &str) -> request::VersionRequest {
        request::VersionRequest {
            client: &self,
            project: project.to_owned(),
            version: version.to_owned(),
        }
    }
    ///Gets all available builds for a project's version.
    pub fn builds(&self, project: &str, version: &str) -> request::BuildsRequest {
        request::BuildsRequest {
            client: &self,
            project: project.to_owned(),
            version: version.to_owned(),
        }
    }
    ///Gets information related to a specific build.
    pub fn build(
        &self,
        project: &str,
        version: &str,
        build: i64,
    ) -> request::BuildRequest {
        request::BuildRequest {
            client: &self,
            project: project.to_owned(),
            version: version.to_owned(),
            build,
        }
    }
    ///Downloads the given file from a build's data.
    pub fn download(&self, args: request::DownloadRequired) -> request::DownloadRequest {
        request::DownloadRequest {
            client: &self,
            project: args.project.to_owned(),
            version: args.version.to_owned(),
            build: args.build,
            download: args.download.to_owned(),
        }
    }
    ///Gets information about a project's version group.
    pub fn family(&self, project: &str, family: &str) -> request::FamilyRequest {
        request::FamilyRequest {
            client: &self,
            project: project.to_owned(),
            family: family.to_owned(),
        }
    }
    ///Gets all available builds for a project's version group.
    pub fn family_builds(
        &self,
        project: &str,
        family: &str,
    ) -> request::FamilyBuildsRequest {
        request::FamilyBuildsRequest {
            client: &self,
            project: project.to_owned(),
            family: family.to_owned(),
        }
    }
}
