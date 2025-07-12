use std::{env, process::exit, thread::sleep, time::Duration};
use carton::operator::{help::operator_help, modpack::{operator_init,operator_modify,operator_pin, operator_unpin, operator_push}, res::{operator_add, operator_delete}};
#[tokio::main]
async fn main(){
    #[cfg(not(debug_assertions))]
    {
        std::panic::set_hook(Box::new(|info|{
            if let Some(s) = info.payload().downcast_ref::<&str>() {
                println!("{s}");
            }
            else if let Some(s)=info.payload().downcast_ref::<String>(){
                println!("{s}")
            }
            else {
                println!("Panic!");
            }
        }));
    }
    let args:Vec<String>=env::args().collect();
    if args.len()<2 {
        println!("No args found!This is a command line program");
        println!("Program will exit in 3 seconds!");
        sleep(Duration::from_secs(3));
        exit(0);
    }
    let arg1=&args[1];
    match arg1.as_str() {
        "init"=>operator_init(),
        "i"=>operator_init(),
        "help"=>operator_help(),
        "h"=>operator_help(),
        "modify"=>operator_modify(),
        "m"=>operator_modify(),
        "pin"=>operator_pin(),
        "p"=>operator_pin(),
        "unpin"=>operator_unpin(),
        "up"=>operator_unpin(),
        "add"=>operator_add(),
        "a"=>operator_add(),
        "delete"=>operator_delete(),
        "d"=>operator_delete(),
        "push"=>operator_push().await,
        "ph"=>operator_push().await,
        _=>{
            println!("Unknown arg: {}",args[1]);
            exit(0);
        }
    }
}