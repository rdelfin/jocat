use crate::{animation::AnimationId, audio, components::Player, prefabs, resources::Level};
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, ProgressCounter},
    audio::{output::Output, Source},
    core::transform::Transform,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, World, WorldExt},
    renderer::{camera::Camera, sprite::SpriteRender},
    utils::application_root_dir,
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use log::info;
use std::fs;

#[derive(Default)]
pub struct Game {
    pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        // Load level data
        load_level(&mut world);
        // Crates new progress counter
        self.progress_counter = Some(Default::default());
        prefabs::load_jocrap(&mut world, self.progress_counter.as_mut().unwrap());
        prefabs::load_player(&mut world, self.progress_counter.as_mut().unwrap());
        prefabs::load_background(&mut world, self.progress_counter.as_mut().unwrap());
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

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data

        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                info!("LOADED");
                let StateData { world, .. } = data;
                // Start the music
                world.exec(
                    |(sounds, storage, audio_output): (
                        ReadExpect<'_, audio::Sounds>,
                        Read<'_, AssetStorage<Source>>,
                        Option<Read<'_, Output>>,
                    )| {
                        audio::play_music(&*sounds, &storage, audio_output.as_deref())
                    },
                );

                // Execute a pass similar to a system
                world.exec(
                    |(entities, animation_sets, players, mut control_sets): (
                        Entities,
                        ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                        ReadStorage<Player>,
                        WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                    )| {
                        // For each entity that has AnimationSet
                        for (entity, animation_set, player) in
                            (&entities, &animation_sets, (&players).maybe()).join()
                        {
                            // Creates a new AnimationControlSet for the entity
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();

                            match player {
                                Some(_) => {
                                    info!("Setting animation to idle");
                                    control_set.add_animation(
                                        AnimationId::Attack,
                                        &animation_set.get(&AnimationId::Attack).unwrap(),
                                        EndControl::Stay,
                                        1.0,
                                        AnimationCommand::Init,
                                    );
                                    control_set.add_animation(
                                        AnimationId::Idle,
                                        &animation_set.get(&AnimationId::Idle).unwrap(),
                                        EndControl::Loop(None),
                                        1.0,
                                        AnimationCommand::Start,
                                    );
                                }
                                None => {
                                    info!("Setting animation to bop");
                                    control_set.add_animation(
                                        AnimationId::JoCrapBop,
                                        &animation_set.get(&AnimationId::JoCrapBop).unwrap(),
                                        EndControl::Loop(None),
                                        1.0,
                                        AnimationCommand::Start,
                                    );
                                }
                            }
                        }
                    },
                );
                // All data loaded
                self.progress_counter = None;
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
    let l: Level = serde_json::from_str(&fs::read_to_string(level_file)?)?;
    world.insert(l);

    Ok(())
}
