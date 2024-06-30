use std::fmt::{Display, Formatter, Result};

use leptos::IntoView;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
#[cfg_attr(
    feature = "ssr",
    sqlx(type_name = "vehicle_category", rename_all = "lowercase")
)]
pub enum Category {
    Suv,
    Van,
    Sedan,
    Coupe,
    Cabrio,
    Motorcycle,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Category::Suv => write!(f, "SUV"),
            Category::Van => write!(f, "Van"),
            Category::Sedan => write!(f, "Sedan"),
            Category::Coupe => write!(f, "Coupe"),
            Category::Cabrio => write!(f, "Cabrio"),
            Category::Motorcycle => write!(f, "Motorcycle"),
        }
    }
}

impl IntoView for Category {
    fn into_view(self) -> leptos::View {
        match self {
            Category::Suv => "Suv".into_view(),
            Category::Van => "Van".into_view(),
            Category::Sedan => "Sedan".into_view(),
            Category::Coupe => "Coupe".into_view(),
            Category::Cabrio => "Cabrio".into_view(),
            Category::Motorcycle => "Motorcycle".into_view(),
        }
    }
}

impl Category {
    pub fn values() -> Vec<Category> {
        vec![
            Category::Suv,
            Category::Van,
            Category::Sedan,
            Category::Coupe,
            Category::Cabrio,
            Category::Motorcycle,
        ]
    }
}
