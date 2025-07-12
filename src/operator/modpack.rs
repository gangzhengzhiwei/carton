use std::{env, fs::read_dir};
use crate::{operator::{help::push_help, mc::input_mc_version, res::{Resource, Source}}, *};

pub fn operator_init(){
    println!("Init a ModPack.");
    println!("Input ModPack name: ");
    let name=read_string();
    let packworkspace=env::current_dir().unwrap().join(&name);
    if packworkspace.exists()&&(!is_dir_empty(&packworkspace)) {
        panic!("Error: Dir {} is not empty!",packworkspace.display());
    }
    let mc_version=input_mc_version();
    println!("ModPack Loader:\n1: Forge\n2: Neoforge\n3: Fabric\n4: Quilt");
    modloader_version_warn();
    let modloader=match_modloader(read_string()).expect("No match index!");
    let modpack=ModPack{name,modpack_version:"0.1.0".to_string(),mc_version,modloader};
    println!("ModPack Info:\nName: {}\nMinecraft Version: {}\nModLoader: {}\nModLoader Version: {}\nType 'y' to confirm,type other to cancel."
        ,modpack.name,modpack.mc_version,modpack.modloader.get_name(),modpack.modloader.get_version());
    if read_string()!="y" {
        canceled();
    }
    //write file
    create_dir_or_else_stop(&packworkspace);
    create_dir_or_else_stop(&packworkspace.join("common"));
    create_dir_or_else_stop(&packworkspace.join("dev"));
    create_dir_or_else_stop(&packworkspace.join("release"));
    write_file_or_else_stop(&packworkspace.join(".gitignore"), "instance.toml\n");
    write_file_or_else_stop(&packworkspace.join("modpack.toml"), toml::to_string(&modpack).unwrap());
    println!("Creating ModPack successfully!");
}
pub fn match_modloader(modloader:String)->Option<ModLoader>{
    match modloader.as_str() {
        "1"=>Some(ModLoader::Forge(forge_version())),
        "2"=>Some(ModLoader::NeoForge(neoforge_version())),
        "3"=>Some(ModLoader::Fabric(fabric_version())),
        "4"=>Some(ModLoader::Quilt(quilt_version())),
        _=>None
    }
}
pub fn forge_version()->String {
    println!("Forge Version: ");
    read_string()
}
pub fn neoforge_version()->String {
    println!("NeoForge Version: ");
    read_string()
}
pub fn fabric_version()->String {
    println!("Fabric Version: ");
    read_string()
}
pub fn quilt_version()->String {
    println!("Quilt Version: ");
    read_string()
}
pub fn operator_pin() {
    let packworkspace=std::env::current_dir().unwrap();
    let modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    println!("Input the game instance version folder:\nnote:Modpack name must matches the game instance folder");
    let path:PathBuf=read_string().try_into().unwrap();
    if !path.exists() {
        panic!("Dir does not exist. Are you sure you input a correct dir?")
    }
    let version_json_raw;
    if path.join(modpack.name.clone()+".json").exists() {
        version_json_raw=fs::read_to_string(&path.join(modpack.name.clone()+".json")).unwrap();
    } else if path.join(modpack.mc_version.clone()+".json").exists() {
        version_json_raw=fs::read_to_string(&path.join(modpack.mc_version.clone()+".json")).unwrap();
    } else {
        panic!("Cannot found version json! Is the dir correct?");
    }
    //get instance mc version
    let version_json:serde_json::Value=serde_json::from_str(version_json_raw.as_str()).expect("Cannot deserializing version json. Is the dir correct?");
    let mut mc_version="-1".to_string();
    let mut modloader=ModLoader::Forge("-1".to_string());
    if let serde_json::Value::String(s)=version_json.get("assets").unwrap() {
        mc_version=s.clone();
    }
    //get instance mc version and modloader
    //This is a shit!!!
    if let Some(value)=version_json.get("arguments") {
        if let Some(value) = value.get("game") {
            if let serde_json::Value::Array(array)=value {
                for (i,v) in array.iter().enumerate() {
                    if let serde_json::Value::String(s) = v {
                        match s.as_str() {
                            "--fml.neoForgeVersion"=>{
                                let modloader_version=if let serde_json::Value::String(version)=array.get(i+1).unwrap() {
                                    version.clone()
                                } else { "-1".to_string() };
                                modloader=ModLoader::NeoForge(modloader_version);
                            }
                            "--fml.forgeVersion"=>{
                                let modloader_version=if let serde_json::Value::String(version)=array.get(i+1).unwrap() {
                                    version.clone()
                                } else { "-1".to_string() };
                                modloader=ModLoader::Forge(modloader_version);
                            }
                            "--fml.mcVersion"=>{
                                mc_version=if let serde_json::Value::String(version)=array.get(i+1).unwrap(){
                                    version.clone()
                                } else { "-1".to_string() }
                            }
                            _=>()
                        }
                    }
                }
            }
        }
    }
    if mc_version=="-1" {
        panic!("Cannot deserializing version json. No Minecraft Version found!");
    }
    if mc_version!=modpack.mc_version {
        panic!("Minecraft version not correct! Expect {} , found {} .",modpack.mc_version,mc_version);
    }
    if !modloader.equals(&modpack.modloader) {
        panic!("Modloader not correct! Expect {} {} , found {} {} .",modpack.modloader.get_name(),modpack.modloader.get_version(),modloader.get_name(),modloader.get_version());
    }
    let game_instance=GameInstance{dir:path.to_str().unwrap().to_string()};
    fs::write(packworkspace.join("instance.toml"),toml::to_string(&game_instance).unwrap()).expect("Error in writing instance.toml file!");
    println!("Pinned successfully!");
}
pub fn operator_unpin() {
    let packworkspace=std::env::current_dir().unwrap();
    let _modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    write_file_or_else_stop(&packworkspace.join("instance.toml"), "");
    println!("Unpined successfully!");
}
pub fn operator_modify() {
    let packworkspace=env::current_dir().unwrap();
    let mut modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    println!("Choose which to modify:\n 1:MC Version\n 2:ModLoader\n 3:ModLoaderVersion\n other:Cancel");
    match read_i64() {
        1=>modify_mc_version(&mut modpack),
        2=>modify_modloader( &mut modpack),
        3=>modify_modloader_version(&mut modpack),
        _=>canceled()
    }
    write_file_or_else_stop(&packworkspace.join("modpack.toml"),toml::to_string(&modpack).unwrap());
}
fn modify_mc_version(modpack:&mut ModPack) {
    println!("Warning!Modify Major MC Version May Cause BAD ERRORS.");
    println!("Current MC Version: {}",modpack.mc_version);
    modpack.mc_version=input_mc_version();
    println!("Successfully modify MC Version to {}",modpack.mc_version);
}
fn modify_modloader(modpack:&mut ModPack) {
    println!("Warning!Modify ModLoader Version May Cause BAD ERRORS.");
    println!("Current ModLoader: {}",modpack.modloader.get_name());
    println!("ModPack Loader:\n1: Forge\n2: Neoforge\n3: Fabric\n4: Quilt");
    modpack.modloader=match_modloader(read_string()).expect("No match index!");
    modify_modloader_version(modpack);
    println!("Successfully modify ModLoader to {} ,version {}",modpack.modloader.get_name(),modpack.modloader.get_version());
}
fn modify_modloader_version(modpack:&mut ModPack) {
    modloader_version_warn();
    modpack.modloader=match modpack.modloader {
        ModLoader::Forge(_)=>{
            ModLoader::Forge(forge_version())
        },
        ModLoader::NeoForge(_)=>{
            ModLoader::NeoForge(neoforge_version())
        },
        ModLoader::Fabric(_)=>{
            ModLoader::Fabric(fabric_version())
        },
        ModLoader::Quilt(_)=>{
            ModLoader::Quilt(quilt_version())
        }
    };
    println!("Successfully modify ModLoader Version to {}",modpack.modloader.get_version());
}
pub async fn operator_push() {
    let args:Vec<String>=std::env::args().collect();
    let packworkspace=std::env::current_dir().unwrap();
    let mut profile=args.get(2).expect("Invalied profile!").to_owned();
    if profile=="-h" || profile=="-help" {
        push_help();
        return;
    }
    let modpack:ModPack=toml::from_str(fs::read_to_string(&packworkspace.join("modpack.toml")).expect("Cannot read modpack.toml.Are you sure there's a project yet?").as_str()).expect("modpack.toml format error!Is this a carton modpack project?");
    let instance:GameInstance=toml::from_str(fs::read_to_string(&packworkspace.join("instance.toml")).expect("Cannot read instance.toml.Maybe you should pin a instance first.").as_str()).unwrap();
    if instance.dir.is_empty() {
        panic!("Not pinned to a game instance yet!");
    }
    let instance_dir:PathBuf=instance.dir.try_into().expect("Bad dir!");
    match profile.as_str() {
        "dev"=>(),
        "d"=>profile="dev".to_string(),
        "release"=>(),
        "r"=>profile="release".to_string(),
        _=>panic!("Invalided profile!")
    };
    //delete old
    let dirs=read_dir(&instance_dir).expect("Cannot read game instance dir!");
    for entry in dirs {
        let dir=entry.unwrap().path();
        let file_name=dir.file_name().unwrap().to_str().unwrap();
        let mut keep=false;
        if file_name.contains(&modpack.name) {
            keep=true;
        }
        if !keep {
            for to_compare in KEEP_IN_PUSH {
                if file_name.contains(to_compare) {
                    keep=true;
                    break;
                }
            }
        }
        if !keep {
            if dir.is_file() {
                fs::remove_file(dir).expect("Error in clean game instance:Cannot remove file!")
            }
            else {
                fs::remove_dir_all(dir).expect("Error in clean game instance:Cannot remove dir!");
            }
        }
    }
    println!("Cleanning game instance successfully!");
    //copy
    copy_dir(&packworkspace.join("common"), &instance_dir).expect("Error in copy dir!");
    copy_dir(&packworkspace.join(&profile), &instance_dir).expect("Error in copy dir!");
    //install mods
    let mut tasks=vec![];
    let client=Client::new();
    let mod_folder=instance_dir.to_owned().join("mods");
    for entry in fs::read_dir(&mod_folder).unwrap() {
        let entry=entry.unwrap();
        let file_name=entry.file_name().into_string().unwrap();
        if entry.file_type().unwrap().is_file()&&file_name.ends_with(".toml") {
            let resource=fs::read_to_string(&mod_folder.join(file_name)).unwrap();
            let resource:Resource=toml::from_str(resource.as_str()).unwrap();
            fs::remove_file(entry.path()).unwrap();
            let mod_folder_clone=mod_folder.clone();
            let client_clone=client.clone();
            match resource.source {
                Source::Curseforge(_curseforge_file) =>todo!(),
                Source::Modrinth(_modrinth_file) => todo!(),
                Source::Url(url_file) => {
                    tasks.push(tokio::spawn(async move{
                        download_file(client_clone,url_file.url,mod_folder_clone,4).await;
                    }));
                },
            }
        }
    }
    for task in tasks {
        task.await.expect("Errror in downloading mods!");
    }
    println!("Pushing {} profile successfully!",profile);
}
const KEEP_IN_PUSH: [&str; 11]=["usernamecache.json","options.txt","usercache.json","resourcepacks","saves","schematics","screenshots","logs","backups","PCL","xaero"];