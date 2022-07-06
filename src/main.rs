use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;
// use router::Router;
// use yew_router::prelude::*;

mod device;

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



#[function_component(App)]
fn app() -> Html {
    
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

    html! {
        <>
        <div>
            <header class="header">
                <img id="findery-logo" src="white-logo.png" alt="Logo"/>
		        <h1 class="logo"><a href="#">{"Findery"}</a></h1>
                <ul class="main-nav">
                    <li><a href="#">{"Add A Device"}</a></li>
                    <li><a href="#">{"Track Device"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                    <li><a href="#">{"Profile"}</a></li>
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

fn main() {
    yew::start_app::<App>();
}