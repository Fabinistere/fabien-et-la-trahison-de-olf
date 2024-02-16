//! Scrolls

use bevy::prelude::*;

use crate::{
    constants::ui::dialogs::SCROLL_ANIMATION_FRAMES_NUMBER,
    ui::dialog::dialog_panel::DialogPanelResources,
};

/// Any scroll should have this component.
///
/// Used to animate scroll.
///
/// # Note
///
/// Can't merge the PlayerScroll and UpperSrcoll int othe Scroll Component,
/// due to quering manners, and update scrolls function being to different
/// from one scroll to another.
///
/// Cause a serie of text is just a monologue and we don't care
/// about the previous text displayed.
/// All choice need to be prompted (not especially on the same page).
#[derive(Component)]
pub struct Scroll {
    pub current_frame: usize,
    pub reverse: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct ScrollTimer(pub Timer);

/// Contains all the line of the current monolog
///
/// Help us keep the `DialogMap` unchanged and progress in a multiline monolog
#[derive(Debug, Reflect, Clone, Default, Resource)]
pub struct Monolog {
    pub source: String,
    pub texts: Vec<String>,
}

#[derive(Component)]
pub struct PlayerChoicePanel;

#[derive(Component)]
pub struct MonologPanel;

/// Contains the state number of the choice: `exit_state` and its position in the ui.
#[derive(Debug, Reflect, PartialEq, Eq, PartialOrd, Ord, Clone, Default, Component)]
pub struct ButtonChoice {
    pub exit_state: usize,
    pub ui_position: usize,
}

impl ButtonChoice {
    pub fn new(ui_position: usize) -> Self {
        ButtonChoice {
            exit_state: usize::default(),
            ui_position,
        }
    }
}

/// # Note
///
/// Waiting for the use of spritesheet in bevy ui.
/// To stop using frame by frame update.
pub fn animate_scroll(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    dialog_panel_resources: Res<DialogPanelResources>,
    mut commands: Commands,
    mut scroll_query: Query<
        (&mut UiImage, &mut Scroll, &mut ScrollTimer, Entity),
        (With<MonologPanel>, Without<PlayerChoicePanel>),
    >,
) {
    for (mut image, mut scroll, mut timer, entity) in scroll_query.iter_mut() {
        timer.tick(time.delta());

        if timer.finished() {
            if scroll.reverse {
                scroll.current_frame -= 1;

                if scroll.current_frame == 0 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            } else {
                scroll.current_frame += 1;

                if scroll.current_frame >= SCROLL_ANIMATION_FRAMES_NUMBER - 1 {
                    commands.entity(entity).remove::<ScrollTimer>();
                }
            }

            image.texture = dialog_panel_resources.scroll_animation[scroll.current_frame].clone();
        }
    }
}
