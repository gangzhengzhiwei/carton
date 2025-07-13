use std::fs::{self};
use serde::{Deserialize, Serialize};
use crate::{operator::help::{add_help, delete_help}, ModPack};

pub fn operator_add() {
    let args:Vec<String>=std::env::args().collect();
    let packworkspace=std::env::current_dir().unwrap();
    let source=args.get(2).expect("Invalied source!");
    if source=="-h" || source=="-help" {
        add_help();
        return;
    }
    let name_or_url=args.get(3).expect("Invalided name!");
    let mut profile=args.get(4).expect("Invalied profile!").to_owned();
    match profile.as_str() {
        "common"=>(),
        "c"=>profile="common".to_string(),
        "dev"=>(),
        "d"=>profile="d".to_string(),
        "release"=>(),
        "r"=>profile="r".to_string(),
        _=>panic!("Invalided profile!")
    }
    let modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    let resource=match source.as_str() {
        "curseforge"=>get_from_curseforge(&modpack, name_or_url),
        "c"=>get_from_curseforge(&modpack, name_or_url),
        "modrinth"=>get_from_modrinth(&modpack, name_or_url),
        "m"=>get_from_modrinth(&modpack, name_or_url),
        "url"=>get_from_url(name_or_url),
        "u"=>get_from_url(name_or_url),
        _=>panic!("Invalied source!")
    };
    let mod_toml=toml::to_string(&resource).unwrap();
    let mod_path=packworkspace.join(profile).join("mods").join(resource.name.to_owned()+".toml");
    if mod_path.exists() {
        panic!("Already has a mod with same name in the profile!");
    }
    fs::write(mod_path, mod_toml).expect("Error in writing mod toml file!");
    println!("Added mod {} sucessfully!",resource.name);
}
/// Split input in %xx;yy;zz
/// 
/// For curseforge source: first_value is project ID,second_value is file ID,third_value is name
/// 
/// For modrinth souce: first_value is version ID,second_value is name,no third_value
/// 
/// For url source: first_value is url,second_value is name,no third_value.
fn split_input(first_value:&mut String,second_value:&mut String,third_value:&mut String,input_str:&String) {
    //This is a shit
    let mut first=false;//before the first '%'
        let mut second=false;//after the second '%'
        for (i,c) in input_str.as_bytes().iter().enumerate() {
            if i==0 {continue;}
            if *c==b'%' {
                if first {second=true;}
                else {first=true;}
            }
            else if first {
                if second {third_value.push(*c as char);}
                else {second_value.push(*c as char);}
            }
            else {first_value.push(*c as char);}
        }
}
fn get_from_curseforge(modpack:&ModPack,input_str:&String)-> Resource {
    let mut project_id=String::new();
    let mut file_id=String::new();
    let mut name=String::new();
    if input_str.starts_with('%') {
        split_input(&mut project_id, &mut file_id, &mut name,input_str);
    }
    else {
        search_from_curseforge(modpack, &input_str);
    }
    Resource {name,source:Source::Curseforge(CurseforgeFile { project_id, file_id })}
}
fn get_from_modrinth(modpack:&ModPack,input_str:&String)-> Resource {
    let mut file_id=String::new();
    let mut name=String::new();
    if input_str.starts_with('%') {
        split_input( &mut file_id, &mut name,&mut String::new(),input_str);
    }
    else {
        search_from_modrinth(modpack, &input_str);
    }
    Resource {name,source:Source::Modrinth(ModrinthFile {  version_id: file_id })}
}
fn get_from_url(input_str:&String)-> Resource {
    let mut url=String::new();
    let mut name=String::new();
    if input_str.starts_with('%') {
        split_input(&mut url, &mut name, &mut String::new(), input_str);
    }
    else {
        panic!("Except a '%' in the beginning!")
    }
    Resource {name,source:Source::Url(UrlFile { url })}
}
fn search_from_curseforge(_modpack:&ModPack,_name:&String)-> Resource {
    todo!()
}
fn search_from_modrinth(_modpack:&ModPack,_name:&String)-> Resource {
    todo!();
}
pub fn operator_delete() {
    let args:Vec<String>=std::env::args().collect();
    let packworkspace=std::env::current_dir().unwrap();
    let name=args.get(2).expect("Invalided name!");
    if name=="-h" || name=="-help" {
        delete_help();
        return;
    }
    let mut profile=args.get(3).expect("Invalided profile!").clone();
    match profile.as_str() {
        "common"=>(),
        "c"=>profile="common".to_string(),
        "dev"=>(),
        "d"=>profile="d".to_string(),
        "release"=>(),
        "r"=>profile="r".to_string(),
        _=>panic!("Invalided profile!")
    }
    let _modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    let file_path=packworkspace.join(&profile).join("mods").join(name.to_owned()+".toml");
    if !file_path.exists() {
        panic!("No mod named '{}' found in profile {} . Is the profile and the name correct?",name,profile);
    }
    fs::remove_file(file_path).expect("Error in deleting file!");
    println!("Deleted '{}' successfully!",name);
}
#[derive(Serialize,Deserialize)]
pub struct Resource {
    pub name:String,
    pub source:Source
}
#[derive(Serialize,Deserialize)]
/// The source of a resource 
pub enum Source {
    Curseforge(CurseforgeFile),
    Modrinth(ModrinthFile),
    Url(UrlFile)
}
#[derive(Serialize,Deserialize)]
pub struct CurseforgeFile{
    pub project_id:String,
    pub file_id:String
}
#[derive(Serialize,Deserialize)]
pub struct ModrinthFile{
    pub version_id:String
}
#[derive(Serialize,Deserialize)]
pub struct UrlFile{
    pub url:String
}