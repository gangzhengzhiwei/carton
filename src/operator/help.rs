pub fn operator_help() {
    println!("Help:\n\
    carton [operator arg] [other args]\n\
    Operator args:
    help(h):Show this help text.
    init(i):Init a new modpack project.
    modify(m):Modify a modpack project's base settings:MC_version,ModLoader and ModLoader Version.
    pin(p):Pin to a game instance for developing and testing a modpack.
    unpin(up)Unpin to a game instace.
    add(a):Add or install a mod to modpack.Type 'add -h' for full help text.
    delete(d):Delete a mod to modpack.Type 'delete -h' for full help text.
    push(ph):Push a profile to the game instance.Type 'push -h' for full help text.
    ")
}
/// Help for add operator.
/// Not for adding a help text!
pub fn add_help() {
    println!("\
    carton add [source] [name or url] [profile]\n\
    Source: Where did you want to add from.
    modrinth(m): Search and download mods from modrinth.com.
    curseforge(c): Search and download mods from curseforge.com.
    url(u): Use url directly.(Not recommend) Warning:Carton will not check it and can not sure whether it is a computer virus!!!\n\
    Name or url: Name of the mod or the url(if use url source).Carton will search and add them.(The searching feature is WIP!)
    You can use the %[project_id]%[file_id]%[mod_name] for curseforge source,%[version_id]&[mod_name] for modrinth source,selecting mod directly.(Carton will not check it.Input carefully!)
    Or use the %[url]%[mod_name] for url source.
    Profile: Which profile you want to add in.
    common(c): The common parts of other profile.
    dev(d): The mods which you only want to use in development.
    release(r): The mods which you only want to use in release.\n\
    \n\
    Special: 'carton add -help' or 'carton add -h' will show this message.
    ")
}
/// Help for delete operator.
/// Not for deleting a help text!
pub fn delete_help() {
    println!("\
    carton delete [name] [profile]\n\
    Name or url: Name of the mod you want to delete.\n\
    Profile: Which profile you want to add in.
    common(c): The common parts of other profile.
    dev(d): The mods which you only want to use in development.
    release(r): The mods which you only want to use in release.\n\
    \n\
    Special: 'carton delete -help' or 'carton delete -h' will show this message.
    ")
}
pub fn push_help() {
    println!("\
    carton push [profile]\n\
    Profile:Which profile you want to push to the game instance.\n\
    This operator will install mods and copy the common folder and the profile folder to game instance.\n\
    Warning:Some files and folder will be deleted first.
    \n\
    Special: 'carton push -help' or 'carton push -h' will show this message.
    ")
}