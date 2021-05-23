use crate::{
    animation::AnimationId,
    components::Player,
    input::{ActionBinding, GameBindingTypes},
};
use amethyst::{
    animation::{get_animation_set, AnimationControlSet},
    derive::SystemDesc,
    ecs::{prelude::*, Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::sprite::SpriteRender,
};

// This is used to keep track of all user input, change animations as needed, and keep
// track of timing of said input
#[derive(Default, SystemDesc)]
pub struct UserInputSystem {
    attack_was_pressed: bool,
}

impl<'s> System<'s> for UserInputSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
        Read<'s, InputHandler<GameBindingTypes>>,
    );

    fn run(&mut self, (entities, players, mut control_sets, input): Self::SystemData) {
        let attack_is_pressed = input
            .action_is_down(&ActionBinding::Attack)
            .unwrap_or(false);

        if !attack_is_pressed && self.attack_was_pressed {
            for (entity, _) in (&entities, &players).join() {
                let control_set = get_animation_set(&mut control_sets, entity).unwrap();

                // Stops the idle animation and starts the attack animation
                control_set.pause(AnimationId::Idle);
                control_set.set_input(AnimationId::Attack, 0.0);
                control_set.start(AnimationId::Attack);
            }
        }

        self.attack_was_pressed = attack_is_pressed;
    }
}
