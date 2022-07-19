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
                <h1 id="about-title">{"ABOUT US"}</h1>
                <div class="about-content">
                    <div class="circle-about-logo">
                        <img id="findery-logo-about" src="white-logo.png" alt="Logo"/>
                    </div>
                    <div class="about-info">
                        <h3 class="about-text" id="company-name">{"Findery - Foundry Digital LLC."}</h3>
                        <h3 class="about-text" id="company-address">{"350 East Ave SUITE 201, Rochester, NY 14604"}</h3>
                        <p class="about-text" id="company-summary">{"The groundbreaking blockchain gps solution, tasked to help you 
                        find your belongings in any environment worldwide. You don’t need bluetooth or WiFi only the people’s network.
                        Helium & Findery allows you to find your items over long distances while providing a environmentally friendly 
                        approach for server-side tracking.
                        "}</p>
                    </div>
                    <div class="about-circle-info">
                        <div class="circle-about-data">
                            <img class="about-images" id="world-wide" src="globepng.png" alt="Worldwide"/>
                        </div>
                        <div class="circle-about-data">
                            <img class="about-images" id="lost-items" src="lostpng.png" alt="Lost Items"/>
                        </div>
                        <div class="circle-about-data">
                            <img class="about-images" id="low-energy" src="sustainable.png" alt="Low Energy"/>
                        </div>
                        <div class="circle-about-data">
                            <img class="about-images" id="iot-img" src="IoT-icon.png" alt="IoT"/>
                        </div>
                        <div class="circle-about-data">
                            <img class="about-images" id="long-distance" src="distanceone.png" alt="Long Distance"/>
                        </div>
                    </div>
                    <div class="description-about-images">
                        <h4 class="desc-img">{"Worldwide"}</h4>
                        <h4 class="desc-img">{"Find Belongings"}</h4>
                        <h4 class="desc-img">{"Low Energy"}</h4>
                        <h4 class="desc-img">{"Multi Purpose IoT"}</h4>
                        <h4 class="desc-img">{"Long Distance"}</h4>
                    </div>
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
                <h1 class="device-title">{"ADD DEVICE"}</h1>
                <div class="add-a-device">
                    <form class="add-device-form">
                        <div class="device-naming-block device-element">
                            <label class="add-device-input device-inp-crit" for="device-name-add">{"DEVICE NAME:"}</label>
                            <input class="add-device-input device-input device-inp-crit" id="device-name-add" name="device-name-add" type="text"/>
                            <label class="add-device-input device-inp-crit" for="device-hash-add">{"IDENTIFICATION HASH:"}</label>
                            <input class="add-device-input device-input device-inp-crit" id="device-hash-add" name="device-hash-add" type="text"/>
                        </div>
                        <div class="device-desc-block device-element">
                            <label class="add-device-input device-desc-crit" for="device-description-add">{"DEVICE DESCRIPTION:"}</label>
                            <textarea class="add-device-input device-input device-desc-crit" id="device-description-add" name="device-description-add" rows="4" cols="50">{""}</textarea>
                        </div>
                        <div class="device-submit-block device-element">
                            <button class="btn-hover search-btn color-5">{"Add Device"}</button>
                        </div>
                    </form>
                </div>
                <h1 class="device-title">{"MY DEVICES"}</h1>
                <div class="my-devices">
                    <div class="device-entry" id="device-1">
                        <p class="device-entry-info" id="device-entry-1">{"Wallet | 882aa4d1ddfffff | This is my wallet and it's very important"}</p>
                        <a href="#" class="remove">{"X"}</a>
                    </div>
                    <div class="device-entry" id="device-1">
                        <p class="device-entry-info" id="device-entry-1">{"Wallet | 882aa4d1ddfffff | This is my wallet and it's very important"}</p>
                        <a href="#" class="remove">{"X"}</a>
                    </div>
                    <div class="device-entry" id="device-1">
                        <p class="device-entry-info" id="device-entry-1">{"Wallet | 882aa4d1ddfffff | This is my wallet and it's very important"}</p>
                        <a href="#" class="remove">{"X"}</a>
                    </div>
                </div>
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