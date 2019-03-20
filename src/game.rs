use amethyst::{
    prelude::*,
    assets::{PrefabLoader, RonFormat},
    ui::{UiCreator},
    renderer::{PosNormTex},
    utils::{
        scene::BasicScenePrefab,
    },
};

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

// What goes here? 
//  - Map, Player, Actors, constants?, 
pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        println!("Started Gameplay");
        // TODO: init camera
        let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sphere.ron", RonFormat, (), ())
        });
        world.create_entity().with(handle).build();
        // TODO: init ui
        // world.exec(|mut creator: UiCreator<'_>| {
        //     creator.create("ui/game.ron", ());
        // });

        // TODO: init char
        // TODO: init room
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // println!("update gameplay");
        let StateData { world, .. } = data;

        Trans::None
    }
}