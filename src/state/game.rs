use crate::{animation::AnimationId, audio, components::Player, prefabs, resources::LevelStart};
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    assets::{AssetStorage, Handle, Prefab},
    audio::{output::Output, Source},
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, WriteStorage},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::{Builder, World, WorldExt},
    renderer::sprite::SpriteRender,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
};
use log::info;
use std::time::Instant;

pub struct Game {
    pub player_handle: Handle<Prefab<prefabs::PlayerPrefab>>,
    pub jocrap_handle: Handle<Prefab<prefabs::PlayerPrefab>>,
    pub background_handle: Handle<Prefab<prefabs::BackgroundPrefab>>,
    pub loaded: bool,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world
            .create_entity()
            .with(self.background_handle.clone())
            .build();
        world
            .create_entity()
            .with(self.player_handle.clone())
            .build();
        world
            .create_entity()
            .with(self.jocrap_handle.clone())
            .build();

        // Start the music
        world.exec(
            |(sounds, storage, audio_output): (
                ReadExpect<'_, audio::Sounds>,
                Read<'_, AssetStorage<Source>>,
                Option<Read<'_, Output>>,
            )| { audio::play_music(&*sounds, &storage, audio_output.as_deref()) },
        );

        // Start the timer
        world.insert(LevelStart(Instant::now()));
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
        let StateData { world, .. } = data;

        if !self.loaded {
            self.setup_animations(world);
        }

        Trans::None
    }
}

impl Game {
    fn setup_animations(&mut self, world: &mut World) {
        // Start the animations
        world.exec(
            #[allow(clippy::type_complexity)]
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
                    self.loaded = true;
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
    }
}
