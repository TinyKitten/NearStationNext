use wasm_bindgen::prelude::*;
use web_sys::Geolocation;
use yew::prelude::*;

use wasm_bindgen::JsCast;

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
    GotError(JsValue),
}

pub struct Home {
    link: ComponentLink<Self>,
    location_error: Option<JsValue>,
    coordinates: Option<(Latitude, Longitude)>,
}
impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            location_error: None,
            coordinates: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HomeMsg::GotLocation(latitude, longitude) => {
                self.coordinates = Some((latitude, longitude));
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
                <h1>{"Home"}</h1>
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
        let error_text = match &self.location_error {
            Some(_) => "Location Error",
            None => "",
        };
        html! {
            <b>{ error_text }</b>
        }
    }
    fn view_location(&self) -> Html {
        let coords_text = match &self.coordinates {
            Some(coords) => format!("{}, {}", coords.0, coords.1),
            None => "".to_string(),
        };
        html! {
            <b>{ coords_text }</b>
        }
    }
}
