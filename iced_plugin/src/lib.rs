use libloading::library_filename;
use std::ffi::OsStr;

pub trait Plugin {
    // type Message: std::fmt::Debug;

    fn run(&self);

    // fn view(&self) -> Element<'_, Self::Message> {
    //     Element::from(iced::widget::text("Hello, World!"))
    // }

    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

pub type CreatePlugin = unsafe fn() -> *mut dyn Plugin;

pub fn load<P: AsRef<OsStr>>(path: P) -> Result<(libloading::Library, Box<dyn Plugin>), libloading::Error> {
    let lib = unsafe { libloading::Library::new(library_filename(path))? };

    let func = unsafe { lib.get::<fn() -> Box<dyn Plugin>>(b"new_plugin")? };
    // let func = unsafe { lib.get::<CreatePlugin>(b"new_plugin")? };
    println!("Got new_plugin");
    println!("Got plugin");

    // let plugin = unsafe { Box::from_raw(func()) };
    let plugin = func();
    Ok((lib, plugin))
}
