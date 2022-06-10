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
    mut query: Query<(&mut GlobalTransform, &mut Slide, Entity)>,
) {
    for (mut transform, mut slide_data, entity) in query.iter_mut() {
        slide_data.elapsed += time.delta();

        let t = slide_data.elapsed.as_secs_f32() / slide_data.duration.as_secs_f32();

        transform.translation = slide_data.start_translation
            + (slide_data.end_translation - slide_data.start_translation)
                * (slide_data.animation_fn)(t);

        if slide_data.elapsed > slide_data.duration {
            commands.entity(entity).remove::<Slide>();
        }
    }
}

#[derive(Component, Debug)]
pub struct UiSlide {
    ui_slide_type: UiSlideType,
    offset: f32,
    orig_offset: f32,
    duration: Duration,
    elapsed: Duration,
    animation_fn: fn(f32) -> f32,
}

#[derive(Debug)]
pub enum UiSlideType {
    ToRight,
    ToLeft,
    ToBottom,
    ToTop,
}

impl UiSlide {
    pub fn new(
        duration: Duration,
        ui_slide_type: UiSlideType,
        offset: f32,
        orig_offset: f32,
        animation_fn: fn(f32) -> f32,
    ) -> Self {
        UiSlide {
            duration,
            ui_slide_type,
            offset,
            orig_offset,
            elapsed: Duration::from_secs(0),
            animation_fn,
        }
    }

    pub fn invert(&mut self) {
        self.ui_slide_type = match self.ui_slide_type {
            UiSlideType::ToRight => UiSlideType::ToLeft,
            UiSlideType::ToLeft => UiSlideType::ToRight,
            UiSlideType::ToBottom => UiSlideType::ToTop,
            UiSlideType::ToTop => UiSlideType::ToBottom,
        };
        self.elapsed = self.duration - self.elapsed;
    }
}

pub fn ui_slide_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Style, &mut UiSlide, Entity)>,
) {
    for (mut style, mut slide_data, entity) in query.iter_mut() {
        slide_data.elapsed += time.delta();

        let t = slide_data.elapsed.as_secs_f32() / slide_data.duration.as_secs_f32();
        let animation_t = (slide_data.animation_fn)(t);
        let mut current_offset = slide_data.orig_offset + slide_data.offset * animation_t;
        if current_offset > slide_data.orig_offset + slide_data.offset {
            current_offset = slide_data.orig_offset + slide_data.offset;
        }

        match slide_data.ui_slide_type {
            UiSlideType::ToRight => style.position.left = Val::Px(current_offset),
            UiSlideType::ToLeft => style.position.right = Val::Px(current_offset),
            UiSlideType::ToBottom => style.position.top = Val::Px(current_offset),
            UiSlideType::ToTop => style.position.bottom = Val::Px(current_offset),
        }

        if slide_data.elapsed > slide_data.duration {
            commands.entity(entity).remove::<UiSlide>();
        }
    }
}
