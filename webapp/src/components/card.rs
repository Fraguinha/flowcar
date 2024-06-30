use leptos::*;

use crate::components::summary::Summary;
use crate::models::vehicle::Vehicle;

#[component]
pub fn Card(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="card mb-3">
            <div class="row g-0">
                <div class="col-md-4">
                    <a href=format!("/vehicle/{}", vehicle.id)>
                        <img src=&vehicle.images[0].src alt=&vehicle.images[0].alt class="img-fluid"/>
                    </a>
                </div>
                <div class="col-md-8">
                    <div class="card-body">
                        <Summary vehicle=vehicle/>
                    </div>
                </div>
            </div>
        </div>
    }
}
