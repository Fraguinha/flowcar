use leptos::*;

use crate::components::feature::Feature;
use crate::models::fuel::Fuel;
use crate::models::vehicle::Vehicle;

#[component]
pub fn Summary(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="row">
            <h3 class="col-8 card-title">
                {&vehicle.title}
            </h3>
            <h3 class="col-4 card-title text-end">
                {format!("{}€", vehicle.price)}
            </h3>
        </div>
        {match vehicle.fuel {
            Fuel::Electric => view! {
                <small class="text-secondary">{format!("{} cv", vehicle.horsepower)}</small>
            },
            Fuel::Diesel | Fuel::Gas | Fuel::Hybrid => view! {
                <small class="text-secondary">{format!("{} cv · {} cm3", vehicle.horsepower, vehicle.displacement)}</small>
            },
        }}
        <div class="mb-3"></div>
        <Feature
            icon="bi bi-calendar-date".to_string()
            text=format!("{}", vehicle.year)
            vertical=false
        />
        <Feature
            icon="bi bi-sign-turn-slight-right".to_string()
            text=format!("{} km", vehicle.mileage)
            vertical=false
        />
        <Feature
            icon={match vehicle.fuel {
                Fuel::Electric => "bi bi-ev-station".to_string(),
                Fuel::Diesel => "bi bi-fuel-pump-diesel".to_string(),
                Fuel::Gas | Fuel::Hybrid => "bi bi-fuel-pump".to_string(),
            }}
            text=format!("{}", vehicle.fuel)
            vertical=false
        />
        <Feature
            icon="bi bi-gear-wide".to_string()
            text=format!("{}", vehicle.transmission)
            vertical=false
        />
    }
}
