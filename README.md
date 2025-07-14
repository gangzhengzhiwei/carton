# Carton

Carton is a minecraft modpack file manager.
### It is in very early-development now.

# Install
First,using rustup (https://rustup.rs/) to install rust toolchain for your machine.

Second,download source and unzip.

Then,use `cargo build --release` to build.Target file will be in the `project_folder/target/release/carton.XXX`.

# Use
A carton modpack project has three profiles: release,dev and common.
Release profile includes files which only will be appear in the release modpack and loadered by user.
Dev profile includes files which only will be appear in the developing environment.(Such as ProbeJs mod)
Common profile includes file which uses in all these two profiles.

`carton init` will init a modpack.(Note: Carton can't check the modloader version whether it is exists!)

`carton add` will add a or search resource(Note: Searching on Curseforge is WIP now.)

`carton push` will push a profile to game instance for developers to test it.But you have to pin first!(Note: Curseforge mod can't be downloaded yet,it's WIP now.)

`carton pin` `carton unpin` will pin or unpin to a game instace.(Note: This feature is WIP.Carton can only check the game version and modloader version on 1.20.1+ Forge and Neoforge.Other version and modloader will cause a error!)

`carton help` will show the help text.


# Features
Search mods on modrinth.

Add and Download mods through url and modrinth.

Push profile to game instance.


# Futures
Search and Install mods on Curseforge.

Pull changes from game instance.

Export the project.

Check modloader version.