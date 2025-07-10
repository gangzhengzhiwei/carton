pub mod operator;
use std::{fs::{self, create_dir_all}, io::stdin, path::{Path, PathBuf}, process::exit};

use serde::{Deserialize, Serialize};
pub const VERSION:&str="0.1.0";
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