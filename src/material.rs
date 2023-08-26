use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "5846207c-e229-477e-ba2f-dec98e205bcd"]
pub struct CoolMaterial {
    #[uniform(0)]
    progress: f32,
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/dialog_box_character_appear.wgsl".into()
    }
}

pub fn material_setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut cool_material_assets: ResMut<Assets<CoolMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        material: cool_material_assets.add(CoolMaterial {
            progress: 0.5,
            texture: asset_server.load("textures/characters/panneau_icon.png"),
        }),
        transform: Transform {
            scale: Vec3::new(1000., 1000., 0.),
            translation: Vec3::new(0., 0., 30.),
            ..Transform::default()
        },
        ..MaterialMesh2dBundle::default()
    });
}
