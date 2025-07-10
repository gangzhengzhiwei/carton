use serde::Deserialize;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{canceled, read_string};

pub fn input_mc_version()->String {
    println!("Downloading Minecraft Version list...");
    let versions=block_in_place(move||{Handle::current().block_on(mc_version())});
    loop {
        println!("Input Minecraft Version.Type 'list' to show all mc version.Input '0' to cancel");
        let input=read_string();
        if input=="0" {
            canceled();
        }
        if input!="list" {
            for v in &versions {
                if input==v.id {
                    return input;
                }
            }
            println!("No match version!Type again or cancel.")
        }
        else {
            for v in &versions {
                println!("{}",v.id);
            }
        }
        
    }
}
pub async fn mc_version()->Vec<Version> {
    let response=reqwest::get("https://piston-meta.mojang.com/mc/game/version_manifest.json").await.expect("Error in downloading MC version_manifest.json!");
    let version_manifest:VersionManifest=response.json().await.unwrap();
    let mut release_versions:Vec<Version>=Vec::with_capacity(20);
    for v in version_manifest.versions{
        if v.r#type=="release" {
            release_versions.push(v);
        }
    }
    release_versions
}
#[derive(Deserialize)]
struct VersionManifest{
    versions:Vec<Version>
}
#[derive(Deserialize)]
pub struct Version{
    pub id:String,
    pub r#type:String
}