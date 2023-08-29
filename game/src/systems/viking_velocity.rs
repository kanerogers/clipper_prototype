use std::time::Instant;

use crate::{config::VIKING_MOVE_SPEED, Game};
use common::{glam::Vec3, hecs::Or, rand};
use components::{BrainwashState, CombatState, Job, Transform, Velocity, Viking};
use rand::Rng;

pub fn update_viking_velocity(game: &mut Game) {
    // Update velocity for Vikings in combat
    update_vikings_in_combat(game);

    // Update velocity for Vikings with jobs
    update_vikings_with_jobs(game);

    // Update velocity for Vikings WITHOUT jobs who are NOT in combat
    update_vikings_without_jobs_or_combat(game);
}

fn update_vikings_in_combat(game: &mut Game) {
    let world = &game.world;

    for (_, (velocity, transform, combat_state)) in world
        .query::<(&mut Velocity, &Transform, &CombatState)>()
        .iter()
    {
        let target_position = game.position_of(combat_state.target);
        velocity.linear = (target_position - transform.position) * VIKING_MOVE_SPEED;
    }
}

fn update_vikings_without_jobs_or_combat(game: &mut Game) {
    let world = &game.world;
    let dave_position = game.dave_position();

    for (_, (viking, velocity, transform)) in world
        .query::<(&mut Viking, &mut Velocity, &Transform)>()
        .without::<Or<&Job, &CombatState>>()
        .iter()
    {
        velocity.linear = match viking.brainwash_state {
            BrainwashState::Free | BrainwashState::BeingBrainwashed(_) => {
                if viking.last_update.elapsed().as_secs_f32() > 1.0 {
                    viking.last_update = Instant::now();
                    random_movement()
                } else {
                    continue;
                }
            }
            BrainwashState::Brainwashed => {
                (dave_position - transform.position).normalize() * VIKING_MOVE_SPEED
            }
        };
    }
}

fn update_vikings_with_jobs(game: &mut Game) {
    let world = &game.world;
    for (_, (velocity, job, transform)) in world
        .query::<(&mut Velocity, &Job, &Transform)>()
        .with::<&Viking>()
        .iter()
    {
        let destination = match job.state {
            components::JobState::GoingToPlaceOfWork => game.position_of(job.place_of_work),
            components::JobState::DroppingOffResource(destination) => game.position_of(destination),
            _ => {
                velocity.linear = Vec3::ZERO;
                continue;
            }
        };

        velocity.linear = (destination - transform.position).normalize() * VIKING_MOVE_SPEED
    }
}

fn random_movement() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0..1.0);
    let z = rng.gen_range(-1.0..1.0);

    Vec3::new(x, 0., z).normalize() * VIKING_MOVE_SPEED
}
