pub mod operator;
use std::{fs::{self, create_dir_all}, io::stdin, path::{Path, PathBuf}, process::exit};

use reqwest::{header::{CONTENT_LENGTH, RANGE, USER_AGENT}, Client, Response};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::operator::res::Source;
pub const VERSION:&str="0.1.2";
#[derive(Deserialize,Serialize)]
pub struct GameInstance{
    dir:String
}
#[derive(Deserialize,Serialize)]
pub struct ModPack{
    name:String,
    modpack_version:String,
    mc_version:String,
    modloader:ModLoader,
}
#[derive(Deserialize,Serialize)]
pub enum ModLoader{
    Forge(String),
    NeoForge(String),
    Fabric(String),
    Quilt(String),
}
impl ModLoader {
    pub fn get_name(&self)->&str {
        match self {
            Self::Forge(_)=>"Forge",
            Self::NeoForge(_)=>"NeoForge",
            Self::Fabric(_)=>"Fabric",
            Self::Quilt(_)=>"Quilt"
        }
    }
    pub fn get_lowercase_name(&self)->&str {
        match self {
            Self::Forge(_)=>"forge",
            Self::NeoForge(_)=>"neoforge",
            Self::Fabric(_)=>"fabric",
            Self::Quilt(_)=>"quilt"
        }
    }
    pub fn get_version(&self)->&String{
        match self {
            Self::Forge(v)=>v,
            Self::NeoForge(v)=>v,
            Self::Fabric(v)=>v,
            Self::Quilt(v)=>v
        }
    }
    pub fn equals(&self,other:&ModLoader)->bool {
        if self.get_name()!=other.get_name() {
            return false;
        }
        if self.get_version()!=other.get_version() {
            return false;
        }
        true
    }
}
/// Use when a operation canceled
pub fn canceled(){
    println!("Canceled!");
    exit(0);
}
pub fn read_string()->String {
    let mut to_ret=String::new();
    stdin().read_line(&mut to_ret).unwrap();
    to_ret=to_ret.trim().to_string();
    to_ret
}
pub fn read_i64()->i64 {
    read_string().parse().expect("Expect an Integer!")
}
pub fn read_usize()->usize {
    read_string().parse().expect("Expect a unsigned number!")
}
///check the dir is empty or the dir does not exist.
/// 
/// return true if the dir is empty or dir does not exist or can not be read.
pub fn is_dir_empty(path: &PathBuf) -> bool {
    match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}
pub fn create_dir_or_else_stop(path: &PathBuf) {
     if !path.exists() {
        if let Err(e) = create_dir_all(&path) {
            println!("Error in creating dir {}.Error: {}",path.display(),e);
            redo_file_init_error(&path);
            exit(1);
        }
    }
}
pub fn write_file_or_else_stop<C: AsRef<[u8]>>(path: &PathBuf, contents: C) {
    if let Err(e) = fs::write(path, contents) {
        println!("Error in writing file.Dir:{} , Error: {}",path.display(),e);
            redo_file_init_error(path);
            exit(1);
    }
}
fn redo_file_init_error(packworkspace:&PathBuf) {
    let _=fs::remove_dir_all(&packworkspace);
}
/// Carton doesn't have the ability to check modloader version yet.
/// Deleted in the future
pub fn modloader_version_warn() {
    println!("warn:Carton doesn't have the ability to check modloader version.Make sure type the right version");
}
/// Copy a dir(Expect itself)
pub fn copy_dir(src:impl AsRef<Path>,dst:impl AsRef<Path>)->std::io::Result<()> {
    copy_dir_inner(src, dst, 1)
}
fn copy_dir_inner(src:impl AsRef<Path>,dst:impl AsRef<Path>,count:i32)->std::io::Result<()> {
    if count>=200 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Too many loops!"));
    }
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry=entry?;
        let file_type=entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_inner(entry.path(), dst.as_ref().join(entry.file_name()), count+1)?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
pub async fn download_file(client:Client,source:Source,output_dir:PathBuf,threads:usize){
    let url;
    let file_name;
    let response;
    let total_size:u64;
    let sha512;
    match source {
        Source::Curseforge(_curseforge_file) => todo!(),
        Source::Modrinth(modrinth_file) => {
            let api_resopnse=client.get(format!("https://api.modrinth.com/v2/version/{}",modrinth_file.version_id))
            .header(USER_AGENT, "gangzhengzhiwei/carton").send().await.expect("No connection to modrinth!");
            let result:serde_json::Value=serde_json::from_str(api_resopnse.text().await.unwrap().as_str()).unwrap();
            let files=result.get("files").unwrap().as_array().unwrap();
            url=files[0].get("url").unwrap().as_str().unwrap().to_string();
            file_name=files[0].get("filename").unwrap().as_str().unwrap().to_string();
            sha512=files[0].get("hashes").unwrap().get("sha512").unwrap().as_str().unwrap().to_string();
            response=client.head(&url).header(USER_AGENT, "gangzhengzhiwei/carton").send().await.expect("No connection");
            total_size = response
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let expect_size=files[0].get("size").unwrap().as_u64().unwrap();
            if total_size!= expect_size{
                panic!("Error in downloading from modrinth version_id: {} .Size not correct.Get {} .Expect {} ",modrinth_file.version_id,total_size,expect_size);
            }
        },
        Source::Url(url_file) => {
            sha512="-1".to_string();
            url=url_file.url;
            response=client.head(&url).header(USER_AGENT, "gangzhengzhiwei/carton").send().await.expect("No connection");
            file_name=prase_filename(&response);
            total_size = response
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
        },
    }
    let threads=if total_size<40960 {
        1
    } else {
        threads
    };
    let file_dir=&output_dir.join(&file_name);
    let chunk_size=total_size/threads as u64;
    let mut file=tokio::fs::File::create(file_dir).await.expect("Cannot open file!");
    let mut tasks=Vec::new();
    for i in 0..threads {
        let start=i as u64 *chunk_size;
        let end=if i==threads-1 {
            total_size
        }
        else {
            (i+1) as u64 * chunk_size-1
        };
        tasks.push(tokio::task::spawn(download_chunk(output_dir.clone(),file_name.clone(),client.clone(),url.clone(),start, end,i)));
    }
    for task in tasks {
        task.await.expect("Error in downloading chunk!");
    }
    for i in 0..threads {
        let tmp_file_dir=&output_dir.join(format!("{}.tmp{}",file_name,i.to_string()));
        let chunk=fs::read(tmp_file_dir).unwrap();
        file.write_all(&chunk).await.unwrap();
        fs::remove_file(tmp_file_dir).unwrap();
    }
    file.flush().await.unwrap();
    let mut file=tokio::fs::File::open(file_dir).await.unwrap();
    if sha512!="-1" {
        let mut hasher=Sha512::new();
        let mut buffer=[0u8;8192];
        loop {
            let byte_read=file.read(&mut buffer).await.expect("Cannot check file sha512!");
            if byte_read==0 {
                break;
            }
            hasher.update(&buffer[..byte_read]);
        }
        let final_sha512=hex::encode(hasher.finalize());
        if sha512!=final_sha512 {
            panic!("Sha512 for {} doesn't match!",file_name);
        }
    }
}
async fn download_chunk(output_dir:PathBuf,file_name:String,client:Client,url:String,start:u64,end:u64,index:usize) {
    let chunk_response=client.get(url).header(RANGE, format!("bytes={}-{}",start,end))
        .header(USER_AGENT, "gangzhengzhiwei/carton").send().await.expect("No connection in chunks!");
    let bytes=chunk_response.bytes().await.unwrap();
    let mut tmpfile=tokio::fs::File::create(output_dir.join(format!("{}.tmp{}",file_name,index.to_string()))).await.expect("Cannot open tmp file!");
    tmpfile.write_all(&bytes).await.unwrap();
}
pub fn prase_filename(response:&Response)->String {
    if let Some(s)=response.headers().get(reqwest::header::CONTENT_DISPOSITION)
    .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split("filename=").nth(1)
                .map(|name| name.trim_matches('"').to_string())
        }) {
            s
    }
    else {
        let final_url=response.url();
        let file_name=final_url.path_segments().and_then(|segments|segments.last()).expect("Error in prase file name!");
        file_name.to_string()
    }
}