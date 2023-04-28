use crate::entities::version::{Version};

#[tracing::instrument]
pub async fn get_version() -> Vec<Version> {
    let mut version_list: Vec<Version> = Vec::new();
    let version = Version {
        name: String::from("Chaineye"),
        description: String::from("链眼社区"),
        version: String::from("V0.1.0"),
    };
    version_list.push(version);
    return version_list;
}

