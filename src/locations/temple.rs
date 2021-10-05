use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{
    player::Player,
    constants::locations::temple::*,
};
use super::{ Location, spawn_collision_cuboid, spawn_child_collision_cuboid };

pub struct TemplePlugin;

impl Plugin for TemplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(Location::Temple)
                    .with_system(setup_temple.system())
                    // .with_system(setup_hitboxes.system())
            )
            .add_system(pillars_position.system());
    }
}

struct Temple;
struct Pillar;

fn pillars_position(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut pillars_query: Query<&mut Children, With<Pillar>>,
    mut collider_pos_query: Query<&mut ColliderPosition>,
) {
    if let Ok(player_transform) = player_query.single() {
        for children in pillars_query.iter_mut() {
            let pos = collider_pos_query.get_mut(children[0]);
            info!("{:?}::{:?}", player_transform.translation, pos);
        }
    }
}

fn setup_temple(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let empty_temple = asset_server.load("textures/temple/empty_temple.png");
    // let pillars = asset_server.load("textures/temple/pillars_and_curtains.png");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(empty_temple.into()),
        transform: Transform::from_matrix(
            Mat4::from_scale_rotation_translation(
                Vec3::splat(TEMPLE_SCALE),
                Quat::default(),
                Vec3::new(0.0, 0.0, TEMPLE_Z),
            )
        ),
        ..SpriteBundle::default()
    }).insert(Temple);

    // commands.spawn_bundle(SpriteBundle {
    //     material: materials.add(pillars.into()),
    //     transform: Transform::from_matrix(
    //         Mat4::from_scale_rotation_translation(
    //             Vec3::splat(TEMPLE_SCALE),
    //             Quat::default(),
    //             Vec3::new(0.0, 0.0, COLUMNS_Z_BACK),
    //         )
    //     ),
    //     ..SpriteBundle::default()
    // }).insert(Pillar);

    let pillar = asset_server.load("textures/temple/pillar.png");
    let pillar_positions = vec![
        Vec3::new(-15.0 * TEMPLE_SCALE, 5.0 * TEMPLE_SCALE, COLUMNS_Z_BACK),
        Vec3::new(15.0 * TEMPLE_SCALE, 5.0 * TEMPLE_SCALE, COLUMNS_Z_BACK),
        Vec3::new(-15.0 * TEMPLE_SCALE, -18.0 * TEMPLE_SCALE, COLUMNS_Z_BACK),
        Vec3::new(15.0 * TEMPLE_SCALE, -18.0 * TEMPLE_SCALE, COLUMNS_Z_BACK),
    ];

    for pos in pillar_positions {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(pillar.clone().into()),
                transform: Transform::from_matrix(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(TEMPLE_SCALE),
                        Quat::default(),
                        pos
                    )
                ),
                ..SpriteBundle::default()
            })
            .insert_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Static,
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                position: Vec2::new(0.0, 0.0).into(),
                ..RigidBodyBundle::default()
            })
            // .insert_bundle((
            //     RigidBodyPositionSync::Discrete,
            // ))
            .with_children(|parent| {
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(8.0, 3.0),
                    position: Vec2::new(pos.x / (TEMPLE_SCALE / 2.0), pos.y / (TEMPLE_SCALE / 2.0) - 12.0).into(),
                    material: ColliderMaterial {
                        friction: 0.0,
                        restitution: 0.0,
                        ..ColliderMaterial::default()
                    },
                    ..ColliderBundle::default()
                });
            })
            .insert(Pillar);

        // spawn_collision_cuboid(&mut commands, 22.0, 1.0, 4.5, 1.5);
    }
}

fn setup_hitboxes(mut commands: Commands) {
    // Top-right pillar
    spawn_collision_cuboid(&mut commands, 22.0, 1.0, 4.5, 1.5);
    // Top-left pillar
    spawn_collision_cuboid(&mut commands, -22.0, 1.0, 4.5, 1.5);
    // Bottom-right pillar
    spawn_collision_cuboid(&mut commands, 22.0, -33.5, 4.5, 1.5);
    // Bottom-left pillar
    spawn_collision_cuboid(&mut commands, -22.0, -33.5, 4.5, 1.5);
}
