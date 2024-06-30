use leptos::*;

use crate::models::vehicle::Vehicle;

#[component]
pub fn Toast(vehicle: Vehicle) -> impl IntoView {
    view! {
        <div class="fixed-bottom p-3 d-flex justify-content-lg-end justify-content-center">
            <div id="vehicle-toast" class="toast align-items-center border-0 show" role="alert" aria-live="assertive" aria-atomic="true" data-bs-autohide="false">
                <div class="d-flex">
                    <div class="toast-body d-flex justify-content-around w-100">
                        <a role="button" class="btn btn-primary w-100 me-3" href=format!("mailto:geral@flowcar.pt?subject=flowcar.pt/vehicle/{}", vehicle.id)>
                            <i class="bi bi-envelope-at-fill"></i>{" Email"}
                        </a>
                        <a role="button" class="btn btn-success w-100" href="tel:939699442">
                            <i class="bi bi-phone-fill"></i>{"Call"}
                        </a>
                    </div>
                    <button type="button" class="btn-close me-2 m-auto" data-bs-dismiss="toast" aria-label="Close"></button>
                </div>
            </div>
        </div>
    }
}
