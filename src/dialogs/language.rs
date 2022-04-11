use bevy::ecs::component::Component;
use serde::Deserialize;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Deserialize, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub enum Language {
    Francais,
    English,
    FabienAncien,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::FabienAncien => write!(f, "Fabien Ancien"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Francais
    }
}
