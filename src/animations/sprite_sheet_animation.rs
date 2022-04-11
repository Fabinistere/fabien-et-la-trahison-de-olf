use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteSheetAnimation {
    pub start_index: usize,
    pub end_index: usize,
    pub timer: Timer,
    pub duration: AnimationDuration,
}

#[derive(Component)]
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
            if sprite.index == animation.end_index {
                if let AnimationDuration::Once = animation.duration {
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
