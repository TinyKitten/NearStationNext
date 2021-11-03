use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::graphql::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Geolocation;

#[wasm_bindgen]
extern "C" {
    type GeolocationCoordinates;

    #[wasm_bindgen(method, getter)]
    fn latitude(this: &GeolocationCoordinates) -> f64;

    #[wasm_bindgen(method, getter)]
    fn longitude(this: &GeolocationCoordinates) -> f64;

    type GeolocationPosition;

    #[wasm_bindgen(method, getter)]
    fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
}

type Latitude = f64;
type Longitude = f64;

pub enum HomeMsg {
    GotLocation(Latitude, Longitude),
    GotStation(Option<graphql_client::Response<nearby_stations::ResponseData>>),
    GotError(JsValue),
}

async fn discover(
    latitude: f64,
    longitude: f64,
) -> Result<graphql_client::Response<nearby_stations::ResponseData>, wasm_bindgen::JsValue> {
    load_nearby_stations(latitude, longitude).await
}

async fn wrap<F: std::future::Future>(f: F, done_cb: yew::Callback<F::Output>) {
    done_cb.emit(f.await);
}

pub struct Home {
    link: ComponentLink<Self>,
    location_error: Option<JsValue>,
    station: Option<nearby_stations::ResponseData>,
}
impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            location_error: None,
            station: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HomeMsg::GotLocation(latitude, longitude) => {
                wasm_bindgen_futures::spawn_local(wrap(
                    discover(latitude, longitude),
                    self.link.callback(
                        |e: Result<
                            graphql_client::Response<nearby_stations::ResponseData>,
                            wasm_bindgen::JsValue,
                        >| match e {
                            Ok(r) => HomeMsg::GotStation(Some(r)),
                            Err(err) => HomeMsg::GotError(err.unchecked_into()),
                        },
                    ),
                ));
                false
            }
            HomeMsg::GotStation(station) => {
                if let Some(s) = station {
                    let json: nearby_stations::ResponseData = s.data.unwrap();
                    self.station = Some(json);
                }
                true
            }
            HomeMsg::GotError(err) => {
                self.location_error = Some(err);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update_current_location()
        }
    }

    fn destroy(&mut self) {}

    fn view(&self) -> Html {
        html! {
            <div>
                {self.view_error_text()}
                {self.view_location()}
            </div>
        }
    }
}
impl Home {
    fn get_geolocation(&self) -> Result<Geolocation, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        window.navigator().geolocation()
    }
    fn update_current_location(&self) {
        let got_location_cb = self.link.callback(|coords: GeolocationCoordinates| {
            HomeMsg::GotLocation(coords.latitude(), coords.longitude())
        });
        let got_error_cb = self.link.callback(|err: JsValue| HomeMsg::GotError(err));
        let geolocation = self
            .get_geolocation()
            .expect("window.navigator.geolocation undefined");
        let success_cb = Closure::wrap(Box::new(move |position| {
            let pos = JsCast::unchecked_into::<GeolocationPosition>(position);
            let coords = pos.coords();
            got_location_cb.emit(coords);
        }) as Box<dyn FnMut(JsValue)>);

        let error_cb = Closure::wrap(Box::new(move |err| {
            got_error_cb.emit(err);
        }) as Box<dyn Fn(JsValue)>);

        geolocation
            .get_current_position_with_error_callback(
                success_cb.as_ref().unchecked_ref(),
                Some(error_cb.as_ref().unchecked_ref()),
            )
            .expect("An error occured on get_current_position_with_error_callback");

        success_cb.forget();
        error_cb.forget();
    }
    fn view_error_text(&self) -> Html {
        if let Some(_) = &self.location_error {
            return html! {
                <b>{"An error occurred!"}</b>
            };
        }
        html! {}
    }
    fn view_location(&self) -> Html {
        if let Some(s) = &self.station {
            let first = s.nearby_stations.as_ref().unwrap().first().unwrap();

            if let Some(data) = first {
                let name = data.name.as_ref().unwrap();
                return html! {<h2>{name}</h2>};
            }
        } else {
            if self.location_error.is_none() {
                return self.view_loading();
            }
        }
        html! {}
    }
    fn view_loading(&self) -> Html {
        html! {<h2>{"Loading..."}</h2>}
    }
}
