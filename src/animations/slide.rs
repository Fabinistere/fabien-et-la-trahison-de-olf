use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct Slide {
    duration: Duration,
    elapsed: Duration,
    start_translation: Vec3,
    end_translation: Vec3,
    animation_fn: fn(f32) -> f32,
}

impl Slide {
    pub fn new(duration: Duration, start: Vec3, end: Vec3, animation_fn: fn(f32) -> f32) -> Self {
        Slide {
            duration,
            elapsed: Duration::from_secs(0),
            start_translation: start,
            end_translation: end,
            animation_fn,
        }
    }

    pub fn invert(&mut self) {
        std::mem::swap(&mut self.start_translation, &mut self.end_translation);
        self.elapsed = self.duration - self.elapsed;
    }
}

pub fn slide_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Slide, Entity)>,
) {
    for (mut transform, mut slide_data, entity) in query.iter_mut() {
        slide_data.elapsed += time.delta();

        let t = slide_data.elapsed.as_secs_f32() / slide_data.duration.as_secs_f32();

        transform.translation *= (slide_data.animation_fn)(t);

        if slide_data.elapsed > slide_data.duration {
            commands.entity(entity).remove::<Slide>();
        }
    }
}
