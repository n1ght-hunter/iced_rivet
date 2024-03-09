use libloading::library_filename;
use std::{any::Any, collections::HashMap, ffi::OsStr, marker::PhantomData};

pub type MessageType = Box<dyn Any + Send>;

pub trait Plugin: Send + Sync {
    fn view(&self) -> iced::Element<'_, MessageType>;

    fn update(&mut self, message: MessageType) -> iced::Command<MessageType>;

    fn subscription(&self) -> Option<iced::Subscription<MessageType>> {
        None
    }

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

pub type PluginFunction = Box<dyn Plugin>;
type CreatePlugin = fn() -> PluginFunction;

pub trait PluginLoader {
    fn plugin_message(id: u16, message: MessageType) -> Self;
}

pub struct PluginHandler<T: 'static + PluginLoader> {
    plugins: HashMap<u16, (libloading::Library, Box<dyn Plugin>)>,
    message: PhantomData<T>
}

impl<T: 'static + PluginLoader> PluginHandler<T> {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            message: PhantomData
        }
    }

    pub fn load<P: AsRef<OsStr>>(&mut self, id: u16, path: P) -> Result<(), libloading::Error> {
        let lib = unsafe { libloading::Library::new(library_filename(path))? };
        println!("Loading plugin");
        let func = unsafe { lib.get::<CreatePlugin>(b"new_plugin")? };
        println!("Plugin loaded");
        let plugin = func();
        println!("Plugin created");
        self.plugins.insert(id, (lib, plugin));
        Ok(())
    }

    pub fn unload(&mut self, id: u16) {
        if let Some((lib, _)) = self.plugins.remove(&id) {
            drop(lib);
        }
    }

    pub fn plugin_update(&mut self, id: u16, message: MessageType) -> iced::Command<T> {
        let update = if let Some((_, plugin)) = self.plugins.get_mut(&id) {
            plugin.update(message)
        } else {
            iced::Command::none()
        };

        update.map(move |message| T::plugin_message(id, message))
    }

    pub fn plugin_view(&self, id: u16) -> iced::Element<'_, T> {
        let view = if let Some((_, plugin)) = self.plugins.get(&id) {
            plugin.view()
        } else {
            iced::widget::text("Plugin not found").into()
        };

        view.map(move |message| T::plugin_message(id, message))
    }

    pub fn drop_libs(&mut self) {
        for (_, (lib, _)) in self.plugins.drain() {
            drop(lib);
        }
    }
}


impl<T: 'static + PluginLoader> Drop for PluginHandler<T> {
    fn drop(&mut self) {
        self.drop_libs();
    }
}
