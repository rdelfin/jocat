use crate::{
    audio, prefabs,
    resources::{Level, ThrownPrefabSet},
    state::Game,
};
use amethyst::{
    assets::{Handle, Prefab, ProgressCounter},
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, World, WorldExt},
    renderer::camera::Camera,
    utils::application_root_dir,
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use log::info;
use std::fs;

#[derive(Default)]
pub struct Loading {
    pub progress_counter: Option<ProgressCounter>,
    pub items_done_last: Option<usize>,
    pub player_handle: Option<Handle<Prefab<prefabs::PlayerPrefab>>>,
    pub jocrap_handle: Option<Handle<Prefab<prefabs::PlayerPrefab>>>,
    pub background_handle: Option<Handle<Prefab<prefabs::BackgroundPrefab>>>,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        // Load level data
        load_level(&mut world).expect("Encountered an issue when loading the level");
        // Crates new progress counter
        self.progress_counter = Some(Default::default());

        // Add throwables
        let throwable_prefab_set =
            ThrownPrefabSet::new(world, self.progress_counter.as_mut().unwrap());
        world.insert(throwable_prefab_set);
        // Load all other prefabs
        self.jocrap_handle = Some(prefabs::load_jocrap(
            &mut world,
            self.progress_counter.as_mut().unwrap(),
        ));
        self.player_handle = Some(prefabs::load_player(
            &mut world,
            self.progress_counter.as_mut().unwrap(),
        ));
        self.background_handle = Some(prefabs::load_background(
            &mut world,
            self.progress_counter.as_mut().unwrap(),
        ));
        // Creates a new camera
        initialise_camera(&mut world);
        audio::initialise_audio(&mut world);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data

        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                info!("LOADED");

                // Switch to the actual game
                return Trans::Switch(Box::new(Game {
                    player_handle: self
                        .player_handle
                        .clone()
                        .expect("Player handle doesn't exist"),
                    jocrap_handle: self
                        .jocrap_handle
                        .clone()
                        .expect("JoCrap handle doesn't exist")
                        .clone(),
                    background_handle: self
                        .background_handle
                        .clone()
                        .expect("Background handle doesn't exist")
                        .clone(),
                }));
            } else {
                let errors = progress_counter.errors();
                if !errors.is_empty() {
                    println!("ERRORS: {:?}", errors);
                }

                let num_finished = progress_counter.num_finished();
                let print = match self.items_done_last {
                    Some(l) => num_finished != l,
                    None => true,
                };
                if print {
                    self.items_done_last = Some(num_finished);
                    let completion_pct =
                        100. * num_finished as f64 / progress_counter.num_assets() as f64;
                    info!(
                        "{:.2}% DONE ({} failed)",
                        completion_pct,
                        progress_counter.num_failed()
                    );
                }
            }
        }
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}

fn load_level(world: &mut World) -> anyhow::Result<()> {
    let level_file = application_root_dir()?.join("assets").join("level.json");
    let mut l: Level = serde_json::from_str(&fs::read_to_string(level_file)?)?;
    l.populate()?;
    world.insert(l);

    Ok(())
}
