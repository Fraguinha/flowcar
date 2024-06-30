use leptos::*;

use crate::models::vehicle::Vehicle;

#[component]
pub fn Gallery(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="bg-light">
            <div class="row offset-lg-3 col-lg-6 offset-1 py-5">
                <h3 class="card-title">{vehicle.title}</h3>
            </div>
            <div class="row offset-lg-3 col-lg-6 pb-5">
                <div id="vehicleCarousel" class="carousel slide p-0" data-bs-ride="carousel">
                    <div class="carousel-inner">
                        {vehicle.images.iter().enumerate().map(|(index, image)| view! {
                            <div class={format!("carousel-item {}", if index == 0 { "active" } else { "" })}>
                                <img src={&image.src} alt={&image.alt} class="d-block w-100"/>
                            </div>
                        }).collect::<Vec<_>>()}
                    </div>
                    <button class="carousel-control-prev" type="button" data-bs-target="#vehicleCarousel" data-bs-slide="prev">
                        <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Previous</span>
                    </button>
                    <button class="carousel-control-next" type="button" data-bs-target="#vehicleCarousel" data-bs-slide="next">
                        <span class="carousel-control-next-icon" aria-hidden="true"></span>
                        <span class="visually-hidden">Next</span>
                    </button>
                </div>
            </div>
            <div class="row offset-lg-3 col-lg-6">
                <div class="d-flex flex-row flex-nowrap overflow-auto">
                    {vehicle.images.iter().enumerate().map(|(index, image)| view! {
                        <div class="p-2">
                            <img src={&image.src} alt={&image.alt} style="height: 100px; cursor: pointer;" data-bs-target="#vehicleCarousel" data-bs-slide-to={index.to_string()} />
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
