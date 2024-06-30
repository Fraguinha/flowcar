use leptos::*;

use crate::components::card::Card;
use crate::components::error::Error;
use crate::models::vehicle::Vehicle;

#[component]
pub fn List(vehicles: Vec<Vehicle>) -> impl IntoView {
    if vehicles.is_empty() {
        view! {
            <>
                <Error
                    title="No Results".to_string()
                    subtitle="Please try searching for something different".to_string()
                    href="/".to_string()
                    text="Back to Home Page".to_string()
                />
            </>
        }
    } else {
        view! {
            <>
                <div class="fullscreen container-fluid container-lg my-3">
                    <div class="row row-cols-1 g-3">
                        {vehicles
                            .iter()
                            .map(|vehicle| {
                                view! {
                                    <div class="col">
                                        <Card vehicle=vehicle.clone()/>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </div>
                </div>
            </>
        }
    }
}
