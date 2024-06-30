use std::fmt::{Display, Formatter, Result};

use leptos::IntoView;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "vehicle_transmission", rename_all = "lowercase")
)]
pub enum Transmission {
    Manual,
    Automatic,
}

impl Display for Transmission {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Transmission::Manual => write!(f, "Manual"),
            Transmission::Automatic => write!(f, "Automatic"),
        }
    }
}

impl IntoView for Transmission {
    fn into_view(self) -> leptos::View {
        match self {
            Transmission::Manual => "Manual".into_view(),
            Transmission::Automatic => "Automatic".into_view(),
        }
    }
}

impl Transmission {
    pub fn values() -> Vec<Transmission> {
        vec![Transmission::Manual, Transmission::Automatic]
    }
}
