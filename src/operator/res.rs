use std::fs::{self};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use tokio::{runtime::Handle, task::block_in_place};
use crate::{canceled, operator::help::{add_help, delete_help}, read_usize, ModPack};

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
    let mod_name=resource.name.replace(r#":"#, r#" "#)+".toml";
    let mod_path=packworkspace.join(profile).join("mods").join(mod_name);
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
    let to_ret:Resource;
    if input_str.starts_with('%') {
        let mut version_id=String::new();
        let mut name=String::new();
        split_input( &mut version_id, &mut name,&mut String::new(),input_str);
        to_ret=Resource {name,source:Source::Modrinth(ModrinthFile {  version_id })}
    }
    else {
        to_ret=block_in_place(move||{Handle::current().block_on(search_from_modrinth(modpack, &input_str))});
    }
    to_ret
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
async fn search_from_modrinth(modpack:&ModPack,name:&String) ->Resource{
    let facets=format!(r#"[["versions:{}"],["categories:{}"],["project_type:mod"]]"#,modpack.mc_version,modpack.modloader.get_lowercase_name());
    let params=[("query",name),("facets",&facets)];
    let client=reqwest::Client::new();
    let mut mods=Vec::new();
    let response=client.get("https://api.modrinth.com/v2/search").header(USER_AGENT, "gangzhengzhiwei/carton")
        .query(&params).send().await.expect("No connection to modrinth!");
    let body:serde_json::Value=response.json().await.unwrap();
    let hits=body.get("hits").unwrap();
    for m in hits.as_array().unwrap() {
        let title=m.get("title").unwrap().as_str().unwrap().to_string();
        let project_id=m.get("project_id").unwrap().as_str().unwrap().to_string();
        mods.push((title,project_id));
    }
    if mods.is_empty() {
        panic!("No one matches! Are you sure the mod is existed?")
    }
    println!("Searched result from modrinth:");
    for (index,(title,_)) in mods.iter().enumerate() {
        println!("{}): {}",index+1,title);
    }
    println!("Type the index to choose.Type '0' to cancel.");
    let index=read_usize();
    if index==0 {
        canceled();
    }
    let (name,project_id)=&mods[index-1];
    let query=[("loaders",format!(r#"["{}"]"#,modpack.modloader.get_lowercase_name())),("game_versions",format!(r#"["{}"]"#,modpack.mc_version))];
    let response=client.get(format!("https://api.modrinth.com/v2/project/{}/version",project_id))
        .query(&query).header(USER_AGENT, "gangzhengzhiwei/carton").send().await.expect("No connection to modrinth");
    let mut versions=Vec::new();
    let body:serde_json::Value=response.json().await.unwrap();
    let array=body.as_array().unwrap();
    for version in array {
        let version_id=version.get("id").unwrap().as_str().unwrap();
        let version_type=version.get("version_type").unwrap().as_str().unwrap();
        let version_number=version.get("version_number").unwrap().as_str().unwrap();
        versions.push((version_id,version_type,version_number));
    }
    if versions.is_empty() {
        panic!("This project has no one matched versions!")
    }
    println!("Versions matched in mod {} .Type a index to choose.Type '0' to cancel.",&name);
    for (i,(_,version_type,version_number)) in versions.iter().enumerate() {
        println!("{}) Version: {} Type: {}",i+1,version_number,version_type);
    }
    let index=read_usize();
    if index==0 {
        canceled();
    }
    let (version_id,_,_)=versions[index-1];
    Resource{name:name.to_owned(),source:Source::Modrinth(ModrinthFile { version_id:version_id.to_string() })}
}
pub fn operator_delete() {
    let args:Vec<String>=std::env::args().collect();
    let packworkspace=std::env::current_dir().unwrap();
    let name=args.get(2).expect("Invalided name!");
    if name=="-h" || name=="-help" {
        delete_help();
        return;
    }
    let name=name.replace(r#":"#,r#" "#);
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