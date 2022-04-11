use bevy::{prelude::*, utils::Duration};

pub enum FadeType {
    FadeIn,
    FadeOut,
}

#[derive(Component)]
pub struct Fade {
    pub current_alpha: f32,
    pub fade_type: FadeType,
    pub total_duration: Duration,
    pub elapsed: Duration,
    pub animation_fn: fn(f32) -> f32,
}

impl std::default::Default for Fade {
    fn default() -> Self {
        Fade {
            current_alpha: 0.0,
            fade_type: FadeType::FadeIn,
            total_duration: Duration::from_secs(1),
            elapsed: Duration::from_secs(0),
            animation_fn: linear,
        }
    }
}

impl Fade {
    pub fn invert(&mut self) {
        self.fade_type = if let FadeType::FadeIn = self.fade_type {
            FadeType::FadeOut
        } else {
            FadeType::FadeIn
        };

        self.elapsed = self.total_duration - self.elapsed;
    }
}

pub fn linear(t: f32) -> f32 {
    t
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t.powi(3)
}

pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - ((t * std::f32::consts::PI) / 2.0).cos()
}

pub fn ease_out_sine(t: f32) -> f32 {
    ((t * std::f32::consts::PI) / 2.0).sin()
}

pub fn fade_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &mut Fade, Entity)>,
) {
    for (mut sprite, mut fade_data, entity) in query.iter_mut() {
        fade_data.elapsed += time.delta();

        let mut t = fade_data.elapsed.as_secs_f32() / fade_data.total_duration.as_secs_f32();

        if let FadeType::FadeIn = fade_data.fade_type {
            t = 1.0 - t;
        }

        fade_data.current_alpha = (fade_data.animation_fn)(t);

        sprite.color.set_a(fade_data.current_alpha);

        if fade_data.elapsed.as_secs_f32() >= fade_data.total_duration.as_secs_f32() {
            commands.entity(entity).remove::<Fade>();
        }
    }
}
