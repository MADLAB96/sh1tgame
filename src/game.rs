use amethyst::{
    prelude::*
};

pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        println!("Started Gameplay");
        // TODO: init camera
        // TODO: init ui
        // TODO: init char
        // TODO: init room
    }
}