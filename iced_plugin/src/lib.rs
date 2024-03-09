use downcast_rs::{impl_downcast, Downcast};
use libloading::library_filename;
use std::{any::Any, ffi::OsStr};

pub trait Plugin: Downcast + Any + Send + Sync {
    // type Message: std::fmt::Debug;

    fn run(&self);

    // fn view(&self) -> Element<'_, Self::Message> {
    //     Element::from(iced::widget::text("Hello, World!"))
    // }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// DO NOT OVERRIDE THIS METHOD
    /// 
    /// This method is used to get the version of the plugin.
    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

impl_downcast!(Plugin);

pub type CreatePlugin = fn() -> Box<dyn Plugin>;

pub fn load<P: AsRef<OsStr>>(
    path: P,
) -> Result<(libloading::Library, Box<dyn Plugin>), libloading::Error> {
    let lib = unsafe { libloading::Library::new(library_filename(path))? };
    println!("Loaded library");

    // let func = unsafe { lib.get::<fn() -> Box<dyn Plugin>>(b"new_plugin")? };
    let func = unsafe { lib.get::<CreatePlugin>(b"new_plugin")? };
    println!("Got new_plugin");

    let plugin = func();
    println!("Got plugin");
    Ok((lib, plugin))
}
