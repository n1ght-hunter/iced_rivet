#[no_mangle]
pub fn new_plugin() -> Box<dyn iced_plugin::Plugin> {
    Box::new(MyPlugin {})
}

pub struct MyPlugin;

// #[derive(Debug)]
// pub enum Message {}

impl iced_plugin::Plugin for MyPlugin {
    fn run(&self) {
        println!("Hello from my plugin!");
    }
    
    // type Message = Message;
}
