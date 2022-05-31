use super::functions::linear;
use bevy::{prelude::*, utils::Duration};

pub enum FadeType {
    FadeIn,
    FadeOut,
}

#[derive(Component)]
pub struct Fade {
    elapsed: Duration,
    fade_type: FadeType,
    duration: Duration,
    animation_fn: fn(f32) -> f32,
}

impl Fade {
    pub fn new(fade_type: FadeType, duration: Duration, animation_fn: fn(f32) -> f32) -> Self {
        Fade {
            elapsed: Duration::from_secs(0),
            fade_type,
            duration,
            animation_fn,
        }
    }

    pub fn invert(&mut self) {
        self.fade_type = match self.fade_type {
            FadeType::FadeIn => FadeType::FadeOut,
            FadeType::FadeOut => FadeType::FadeIn,
        };

        self.elapsed = self.duration - self.elapsed;
    }
}

impl std::default::Default for Fade {
    fn default() -> Self {
        Fade {
            fade_type: FadeType::FadeIn,
            duration: Duration::from_secs(1),
            elapsed: Duration::from_secs(0),
            animation_fn: linear,
        }
    }
}

pub fn fade_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Fade, Entity)>,
) {
    for (mut sprite, mut fade_data, entity) in query.iter_mut() {
        fade_data.elapsed += time.delta();

        let mut t = fade_data.elapsed.as_secs_f32() / fade_data.duration.as_secs_f32();

        if let FadeType::FadeIn = fade_data.fade_type {
            t = 1.0 - t;
        }

        sprite.color.set_a((fade_data.animation_fn)(t));

        if fade_data.elapsed >= fade_data.duration {
            commands.entity(entity).remove::<Fade>();
        }
    }
}
