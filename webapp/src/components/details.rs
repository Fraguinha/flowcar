use leptos::*;

use crate::models::fuel::Fuel;
use crate::models::model::Model;
use crate::models::vehicle::Vehicle;

#[component]
pub fn Details(vehicle: Vehicle, model: Model) -> impl IntoView {
    view! {
        <div class="row">
            <h3 class="col card-title my-3">
                {"Details"}
            </h3>
        </div>
        <div class="row">
            <div class="col-6">
                <strong>{"Make"}</strong>
            </div>
            <div class="col-6">
                {model.make}
            </div>
            <div class="col-6">
                <strong>{"Model"}</strong>
            </div>
            <div class="col-6">
                {model.model}
            </div>
            <div class="col-6">
                <strong>{"Year"}</strong>
            </div>
            <div class="col-6">
                {vehicle.year}
            </div>
            <div class="col-6">
                <strong>{"Mileage"}</strong>
            </div>
            <div class="col-6">
                {format!("{} km", vehicle.mileage)}
            </div>
            <div class="col-6">
                <strong>{"Horsepower"}</strong>
            </div>
            <div class="col-6">
                {format!("{} cv", vehicle.horsepower)}
            </div>
            {match vehicle.fuel {
                Fuel::Electric => view! { <>{}</> },
                Fuel::Diesel | Fuel::Gas | Fuel::Hybrid => view! {
                    <>
                        <div class="col-6">
                            <strong>{"Displacement"}</strong>
                        </div>
                        <div class="col-6">
                            {format!("{} cm3", vehicle.displacement)}
                        </div>
                    </>
                },
            }}
            <div class="col-6">
                <strong>{"Fuel"}</strong>
            </div>
            <div class="col-6">
                {vehicle.fuel.into_view()}
            </div>
            <div class="col-6">
                <strong>{"Transmission"}</strong>
            </div>
            <div class="col-6">
                {vehicle.transmission.into_view()}
            </div>
        </div>
        <div class="row">
            {if vehicle.extra.is_empty() {
                view! { <>{}</> }
            } else {
                view! {
                    <>
                        <h3 class="col card-title my-3">
                            {"Features"}
                        </h3>
                    </>
                }
            }}
        </div>
        <div class="row">
            {vehicle.extra
                .iter()
                .map(|feature| view! {
                    <div class="col-12">{feature}</div>
                })
                .collect::<Vec<_>>()
            }
        </div>
    }
}
