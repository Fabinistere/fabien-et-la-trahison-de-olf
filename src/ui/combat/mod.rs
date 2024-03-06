use bevy::prelude::*;

use crate::{
    characters::FabiensInfos,
    combat::{
        // tactical_position,
        CombatState
    },
    HUDState,
};

use self::{combat_system::{ActionHistory, ActionsLogs, LastTurnActionHistory}, combat_panel::{CharacterSheetElements, CombatWallResources, CharacterSheetAssetsResources}, log_cave::CombatLogResources};

pub mod character_sheet;
pub mod combat_panel;
pub mod combat_system;
pub mod initiative_bar;
pub mod log_cave;
pub mod player_interaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum UiLabel {
    // /// everything that handles textures
    // Textures,
    /// everything that updates player state
    Player,
    ///
    Display,
}

pub struct UiCombatPlugin;

impl Plugin for UiCombatPlugin {
    /// # Note
    /// 
    /// `.run_if(in_state(HUDState::CombatWall))` is NOT implied by any
    /// `.in_set(CombatState::...)`
    /// 
    /// REFACTOR: Restrict to CombatWall/LogCave - Add everywhere it's not implied `.run_if(in_state(HUDState::CombatWall))` 
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ActionsLogs(String::from("---------------\nActions Logs:")))
            .insert_resource(ActionHistory(String::from("---------------\nCurrent Turn Actions:")))
            .insert_resource(LastTurnActionHistory(String::from("---------------\nLast Turn Actions:")))
            // `CharacterSheetElements` will be initialized in `ui::combat_panel::setup()`
            .insert_resource(CharacterSheetElements::default())
            .init_resource::<FabiensInfos>()
            .init_resource::<CombatWallResources>()
            .init_resource::<CombatLogResources>()
            .init_resource::<CharacterSheetAssetsResources>()

            .add_event::<combat_system::UpdateUnitSelectedEvent>()
            .add_event::<combat_system::UpdateUnitTargetedEvent>()
            
            /* -------------------------------------------------------------------------- */
            /*                         --- Player Input Global ---                        */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                Update,
                (
                    // REFACTOR: move create_dialog_panel_on_key_press to player_interaction
                    combat_panel::create_combat_panel_on_key_press,
                    combat_panel::create_combat_panel_on_combat_event,
                    player_interaction::mouse_scroll,
                    player_interaction::cancel_last_input,
                    player_interaction::select_unit_by_mouse, // .run_if(in_state(HUDState::CombatWall))
                ).in_set(UiLabel::Player)
            )
            .add_systems(Update, player_interaction::action_button.after(initiative_bar::action_visibility))
            
            /* -------------------------------------------------------------------------- */
            /*                                   States                                   */
            /* -------------------------------------------------------------------------- */

            // .add_systems(Startup, combat_panel::global_ui_setup)
            .add_systems(OnEnter(HUDState::CombatWall), (
                combat_panel::global_ui_setup,
                combat_panel::combat_wall_setup
            ))
            .add_systems(OnExit(HUDState::CombatWall), combat_panel::cleanup)
            
            .add_systems(
                OnEnter(HUDState::LogCave),
                (
                    log_cave::setup,
                    combat_system::current_action_displayer.after(log_cave::setup),
                )
            )
            .add_systems(OnExit(HUDState::LogCave), log_cave::cleanup)

            /* -------------------------------------------------------------------------- */
            /*                            --- Limited Phase ---                           */
            /* -------------------------------------------------------------------------- */
            
            .add_systems(
                Update,
                combat_system::update_alterations_status.after(CombatState::AlterationsExecution)
            )
            .add_systems(
                Update,
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),

                    player_interaction::end_of_turn_button.in_set(UiLabel::Player),
                    // prevent clicking a MiniCharSheet while already in "Character Sheet Focused", which cover the MiniCS.   
                    player_interaction::mini_character_sheet_interact.in_set(UiLabel::Player),
                    log_cave::cave_ladder.in_set(UiLabel::Player),
                )
                    .in_set(CombatState::SelectionCaster)
            )
            // in SkillPhase: There is one selected
            .add_systems(
                Update,
                (
                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),

                    // cancel the current action if imcomplete -----vvv
                    player_interaction::end_of_turn_button.in_set(UiLabel::Player),
                    player_interaction::select_skill.in_set(UiLabel::Player),
                    player_interaction::browse_character_sheet.in_set(UiLabel::Player),
                    // FIXME: In SelectionSkill, the end_of_turn trigger twice, CombatStates -> derive States could fix that but having so much States might not be so cool

                    character_sheet::update_headers,
                    character_sheet::update_weapon_displayer,
                    character_sheet::update_caster_stats_panel.after(UiLabel::Player),
                )
                    .in_set(CombatState::SelectionSkill)
                    // .run_if(in_state(HUDState::CombatWall)) // TOTEST: Keep this schedule may crash the system (event handler etc)
            )
            .add_systems(
                Update,
                (
                    combat_system::target_selection,
                    combat_system::update_targeted_unit.after(UiLabel::Player),

                    // switch to a new action ----vvv
                    player_interaction::select_skill,
                    player_interaction::end_of_turn_button,

                    // character_sheet::update_headers,
                    // character_sheet::update_weapon_displayer,
                    character_sheet::update_caster_stats_panel.after(UiLabel::Player),
                )
                    .in_set(CombatState::SelectionTarget)
            )
            // .add_systems(
            //     ().run_if(in_initiative_phase)
            // )
            .add_systems(
                Update,
                // always run
                combat_system::update_alterations_status.after(CombatState::ExecuteSkills)
            )

            .add_systems(
                Update,
                (
                    player_interaction::browse_character_sheet,
                    player_interaction::end_of_turn_button,

                    combat_system::caster_selection,
                    combat_system::update_selected_unit.after(UiLabel::Player),

                    character_sheet::update_headers,
                    character_sheet::update_caster_stats_panel.after(UiLabel::Player),
                    character_sheet::update_weapon_displayer,
                )
                    .in_set(CombatState::BrowseEnemySheet)
            )
            // .add_systems(
            //     ().run_if(in_evasive_phase)
            // )
            
            /* -------------------------------------------------------------------------- */
            /*                            -- DEBUG DISPLAYER --                           */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                Update,
                (
                    combat_system::update_combat_phase_displayer,
                    combat_system::current_action_formater
                        .after(CombatState::RollInitiative)
                        .before(CombatState::ExecuteSkills),
                    character_sheet::update_target_stats_panel
                        .after(UiLabel::Player),
                    initiative_bar::action_visibility
                        .after(CombatState::SelectionSkill)
                        .after(CombatState::SelectionTarget),
                    character_sheet::skill_visibility
                        .after(CombatState::SelectionCaster),
                )
                    .in_set(UiLabel::Display)
            )
            .add_systems(
                Update,
                (
                    combat_system::current_action_displayer
                        .after(combat_system::current_action_formater),
                    combat_system::last_action_displayer
                        .after(CombatState::ExecuteSkills),
                    combat_system::actions_logs_displayer
                        .after(CombatState::RollInitiative)
                        .after(CombatState::ExecuteSkills),
                )
            )

            /* -------------------------------------------------------------------------- */
            /*                                --- COLOR ---                               */
            /* -------------------------------------------------------------------------- */
            .add_systems(
                Update,
                (
                    character_sheet::skill_color,
                    player_interaction::button_system,
                )
                    .after(UiLabel::Display)
            )

            /* -------------------------------------------------------------------------- */
            /*                                   Window                                   */
            /* -------------------------------------------------------------------------- */
            // .add_systems(
            //     Update,
            //     (
            //         tactical_position::detect_window_tactical_pos_change,
            //         tactical_position::update_character_position
            //             .after(tactical_position::detect_window_tactical_pos_change),
            //     )
            // )
            ;
    }
}

