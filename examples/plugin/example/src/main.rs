use iced_plugin::load;


fn main() {
    
    // load("target/release/my_plugin").unwrap();
    let (lib, plugin) = load("target/debug/my_plugin").unwrap();
    println!("Loaded plugin");
    println!("Plugin version: {}", plugin.version());
    plugin.run();
    println!("Ran plugin");
}
