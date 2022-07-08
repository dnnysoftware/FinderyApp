use std::time::SystemTime;
use futures::StreamExt;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document, from_document};
use mongodb::options::*;
use mongodb::cursor::Cursor;

mod device;

#[wasm_bindgen(module = "/js/devicemap.js")]
extern "C"{
    fn getDeviceFromSearch();
}

#[wasm_bindgen(module = "/js/devicemap.js")]
extern "C" {
    fn clearMapSearch();
    fn updateMap(url_together: String, device_name: String, lat: f32, long: f32, description: String, battery_life: f32, time: String);
}

fn build_device(name: &String) -> device::Device{

    device::Device {

        device_name: ["Looking for ", &name, "... "].concat(),
        description: String::from("Searching..."),
        coordinates: (0f32,0f32),
        battery_life: (0f32),
        time: SystemTime::now(), //until actual device data after
        device_ID: 1, //Hard coded for now
    }

}

// To add data to database from user input: see https://www.youtube.com/watch?v=fm6oinNergw at 20 minutes

async fn search(name: String)-> Result<device::Device, mongodb::error::Error>{

    //before we get data

    let mut dev = build_device(&name);

    //after we get data

    let client_uri = String::from("mongodb+srv://FinderyApp:FinderyApp@cluster0.f6fjujq.mongodb.net");

    //Connecting to database
    let client_options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(client_options)?;
    let modules = client.database("FinderyApp").collection("Modules");

    //Finding the correct element
    let query = doc!{"TrackerID": dev.device_ID};
    let find_options = FindOptions::builder().sort(doc!{"TrackerID": dev.device_ID}).build();
    let mut cursor = modules.find(query, find_options).await?;

    //Getting the values
    let mut counter = 0;
    while let Some(result) = cursor.next().await {
        let dev = from_document(result?);
    }

    //returning initial device
    dev

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