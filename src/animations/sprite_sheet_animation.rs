use bevy::prelude::*;

#[derive(Reflect, Component)]
pub struct SpriteSheetAnimation {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
    pub duration: AnimationDuration,
}

#[derive(Reflect, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum AnimationDuration {
    Infinite,
    Once,
}

pub fn animate_sprite_sheet(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteSheetAnimation, &mut TextureAtlasSprite)>,
) {
    for (entity, mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if sprite.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    sprite.index = animation.start_index;
                }
            } else {
                sprite.index += 1;
            }
        }
    }
}

pub fn animate_ui_atlas(
    mut commands: Commands,
    time: Res<Time>,
    mut atlas_images: Query<(Entity, &mut SpriteSheetAnimation, &mut UiTextureAtlasImage)>,
) {
    for (entity, mut animation, mut atlas_image) in atlas_images.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            if atlas_image.index >= animation.end_index {
                if animation.duration == AnimationDuration::Once {
                    commands.entity(entity).remove::<SpriteSheetAnimation>();
                } else {
                    atlas_image.index = animation.start_index;
                }
            } else {
                atlas_image.index += 1;
            }
        }
    }
}
