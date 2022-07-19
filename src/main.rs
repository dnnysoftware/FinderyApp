use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;

mod device;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/devices")]
    Devices,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn build_device(name: &String) -> device::Device{

    device::Device {

        device_name: name.to_string(),
        description: String::from("This is my wallet and it's very important"),
        coordinates: (43.1566,-77.6088),
        battery_life:(74.0),
        time: String::from("07-05-2022 10:32"), //until actual device data after
        device_id: 1, //Hard coded for now
    }
}

#[wasm_bindgen(module = "/js/devicemap.js")]
extern "C" {
    fn clearMapSearch();
    fn updateMap(url_together: String, device_name: String, lat: f32, long: f32, description: String, battery_life: f32, time: String);
}

#[function_component(About)]
fn about() -> Html {
    let history_about = use_history().unwrap();
    let history_home = use_history().unwrap();
    let history_device = use_history().unwrap();

    let about_callback = Callback::from(move |_| history_about.push(Route::About));
    let track_callback = Callback::from(move |_| history_home.push(Route::Home));
    let device_callback = Callback::from(move |_| history_device.push(Route::Devices));
    html! {
        <>
        <div>
            <header class="header">
                <img id="findery-logo" src="white-logo.png" alt="Logo"/>
		        <h1 class="logo"><a href="#">{"Findery"}</a></h1>
                <ul class="main-nav">
                    <li><a onclick={device_callback}>{"Add A Device"}</a></li>
                    <li><a onclick={track_callback}>{"Track Device"}</a></li>
                    <li><a onclick={about_callback}>{"About"}</a></li>
                    <li><a>{"Profile"}</a></li>
                </ul>
	        </header> 
            <div class="about-main">
                <h1 id="about-title">{"About Us"}</h1>
                <div class="about-content">
                </div>
            </div>
        </div>
        </>
    }
}

#[function_component(Devices)]
fn devices() -> Html {
    let history_about = use_history().unwrap();
    let history_home = use_history().unwrap();
    let history_device = use_history().unwrap();

    let about_callback = Callback::from(move |_| history_about.push(Route::About));
    let track_callback = Callback::from(move |_| history_home.push(Route::Home));
    let device_callback = Callback::from(move |_| history_device.push(Route::Devices));
    html! {
        <>
        <div>
            <header class="header">
                <img id="findery-logo" src="white-logo.png" alt="Logo"/>
		        <h1 class="logo"><a href="#">{"Findery"}</a></h1>
                <ul class="main-nav">
                    <li><a onclick={device_callback}>{"Add A Device"}</a></li>
                    <li><a onclick={track_callback}>{"Track Device"}</a></li>
                    <li><a onclick={about_callback}>{"About"}</a></li>
                    <li><a>{"Profile"}</a></li>
                </ul>
	        </header> 
            <div class="device-main">
            </div>
        </div>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {

    let clear = Callback::from(|_|{
        clearMapSearch();
    });

    let map_url: &str = "";

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();
    let onclick = Callback::from(move |_| {
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let device_name = input.value();
        let derived_device = build_device(&device_name);
        let lat = derived_device.coordinates.0;
        let long = derived_device.coordinates.1;
        let map_url_string: String = format!("https://maps.google.com/maps?q={},{}&hl=en&z=14&amp;output=embed", lat, long);
        let url_together = format!("{}{}", map_url, map_url_string);
        updateMap(url_together, derived_device.device_name, derived_device.coordinates.0, derived_device.coordinates.1 , derived_device.description, derived_device.battery_life, derived_device.time);
        
    });

    let history_about = use_history().unwrap();
    let history_home = use_history().unwrap();
    let history_device = use_history().unwrap();

    let about_callback = Callback::from(move |_| history_about.push(Route::About));
    let track_callback = Callback::from(move |_| history_home.push(Route::Home));
    let device_callback = Callback::from(move |_| history_device.push(Route::Devices));
     html! {
        <>
        <div>
            <header class="header">
                <img id="findery-logo" src="white-logo.png" alt="Logo"/>
		        <h1 class="logo"><a href="#">{"Findery"}</a></h1>
                <ul class="main-nav">
                    <li><a onclick={device_callback}>{"Add A Device"}</a></li>
                    <li><a onclick={track_callback}>{"Track Device"}</a></li>
                    <li><a onclick={about_callback}>{"About"}</a></li>
                    <li><a>{"Profile"}</a></li>
                </ul>
	        </header> 
            <div class="search-page">
                <div class="map-search-field">
                    <label class="search-map-form" for="device-name">{"SEARCH DEVICE:"}</label>
                    <input ref={input_ref_outer.clone()} class="search-map-form" type="text" id="device-name" name="device-search-box"/>
                    <button class="search-map-form btn-hover search-btn color-5" id="search-device-btn" name="search-device-btn" {onclick}>{"Search"}</button>
                    <button class="search-map-form btn-hover search-btn color-5" id="clear-map-data" name="clear-map-data" onclick={clear}>{"Clear"}</button>    
                </div>
                <div class="search-results">
                    <div id="mapping-structure" class="mapping search-block">
                        <iframe id="map-visual" width="500" height="450" frameborder="5" scrolling="yes" marginheight="0" marginwidth="0" src="https://maps.google.com/maps?q=40.7128,-74.0060&t=&z=15&ie=UTF8&iwloc=&output=embed" />
                    </div>
                    <div class="gps-data-output-table search-block">
                        <table>
                            <tr>
                                <th>{"Device Name:"}</th>
                                <td><input class="search-results" id="search-name-result" type="text"/></td>
                            </tr>
                            <tr>
                                <th>{"Description:"}</th>
                                <td><input class="search-results" id="search-desc-result" type="text"/></td>
                            </tr>
                            <tr>
                                <th>{"Latitude:"}</th>
                                <td><input class="search-results" id="search-lat-result" type="text"/></td>
                            </tr>
                            <tr>
                                <th>{"Longitude:"}</th>
                                <td><input class="search-results" id="search-long-result" type="text"/></td>
                            </tr>
                            <tr>
                                <th>{"Battery Life:"}</th>
                                <td><input class="search-results" id="search-life-result" type="text"/></td>
                            </tr>
                            <tr>
                                <th>{"Time Stamp:"}</th>
                                <td><input class="search-results" id="search-time-result" type="text"/></td>
                            </tr>
                        </table>
                    </div>
                </div>
            </div>
        </div>
       </>
     }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
        },
        Route::About => html! {
            <About />
        },
        Route::Devices => html! {
            <Devices />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}