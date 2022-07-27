mod curtains;
pub mod first_corridor;
pub mod main_room;
pub mod second_corridor;
pub mod secret_room;

use super::{spawn_collision_cuboid, Location};
use crate::{constants::locations::temple::*, GameState};
use bevy::{ecs::schedule::ShouldRun, prelude::*};

#[derive(Component)]
pub struct Temple;

#[derive(Component, Deref, DerefMut)]
pub struct ZPosition(f32);

// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerLocation {
    Temple,
    SecretRoom,
}

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerLocation::Temple)
            .add_state(curtains::PlayerCurtainsPosition::Below)
            .add_event::<secret_room::SecretRoomTriggerEvent>()
            .add_event::<second_corridor::DoorInteractEvent>()
            .add_event::<main_room::EnterMainRoomEvent>()
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(spawn_hitboxes)
                    .with_system(main_room::setup_main_room)
                    .with_system(first_corridor::setup_first_corridor)
                    .with_system(second_corridor::setup_second_corridor)
                    .with_system(secret_room::setup_secret_room)
                    .with_system(curtains::setup_curtains),
            )
            .add_system_set(
                SystemSet::on_enter(PlayerLocation::SecretRoom)
                    .with_system(secret_room::remove_secret_room_cover),
            )
            .add_system_set(
                SystemSet::on_exit(PlayerLocation::SecretRoom)
                    .with_system(secret_room::add_secret_room_cover),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(run_if_in_temple)
                    .with_system(main_room::enter_main_room)
                    .with_system(main_room::pillars_position)
                    .with_system(main_room::throne_position)
                    .with_system(curtains::curtains_animation)
                    .with_system(curtains::curtains_z_position)
                    .with_system(secret_room::secret_room_trigger)
                    .with_system(secret_room::olf_cat_animation)
                    .with_system(first_corridor::open_close_door)
                    .with_system(second_corridor::open_close_door)
                    .with_system(second_corridor::door_interact),
            );
    }
}

pub fn run_if_in_temple(
    location: Res<State<Location>>,
    game_state: Res<State<GameState>>,
) -> ShouldRun {
    if location.current() == &Location::Temple && game_state.current() == &GameState::Playing {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn setup_temple(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("textures/temple/background.png");
    let corridor_doors = asset_server.load("textures/temple/corridor_doors.png");

    commands.spawn_bundle(SpriteBundle {
        texture: background,
        transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
        ..SpriteBundle::default()
    });

    commands.spawn_bundle(SpriteBundle {
        texture: corridor_doors,
        transform: Transform::from_xyz(0.0, 0.0, CORRIDOR_DOORS_Z),
        ..SpriteBundle::default()
    });
}

fn spawn_hitboxes(mut commands: Commands) {
    // Left wall
    spawn_collision_cuboid(&mut commands, -1320.0, 80.0, 10.0, 1455.0);
    // Right wall
    spawn_collision_cuboid(&mut commands, 860.0, 80.0, 10.0, 1455.0);
    // Left side of top wall
    spawn_collision_cuboid(&mut commands, -895.0, 975.0, 415.0, 30.0);
    // Right side of top wall
    spawn_collision_cuboid(&mut commands, 225.0, 975.0, 625.0, 30.0);
    // Left side of bottom wall
    spawn_collision_cuboid(&mut commands, -815.0, -805.0, 495.0, 30.0);
    // Right side of bottom wall
    spawn_collision_cuboid(&mut commands, 355.0, -805.0, 495.0, 30.0);
    // Throne seat
    spawn_collision_cuboid(&mut commands, -230.0, 860.0, 70.0, 40.0);
    // Throne front of seat
    spawn_collision_cuboid(&mut commands, -230.0, 810.0, 50.0, 10.0);
    // Throne front of front of seat
    spawn_collision_cuboid(&mut commands, -230.0, 790.0, 30.0, 10.0);
    // Throne bump left 1
    spawn_collision_cuboid(&mut commands, -560.0, 875.0, 1.0, 60.0);
    // Throne bump right 1
    spawn_collision_cuboid(&mut commands, 100.0, 875.0, 1.0, 60.0);
    // Throne bump left 2
    spawn_collision_cuboid(&mut commands, -540.0, 785.0, 1.0, 30.0);
    // Throne bump right 2
    spawn_collision_cuboid(&mut commands, 80.0, 785.0, 1.0, 30.0);
    // Throne bump left 3
    spawn_collision_cuboid(&mut commands, -520.0, 710.0, 1.0, 45.0);
    // Throne bump right 3
    spawn_collision_cuboid(&mut commands, 60.0, 710.0, 1.0, 45.0);
    // Throne bump left 4
    spawn_collision_cuboid(&mut commands, -460.0, 635.0, 1.0, 30.0);
    // Throne bump right 4
    spawn_collision_cuboid(&mut commands, 0.0, 635.0, 1.0, 30.0);
}
