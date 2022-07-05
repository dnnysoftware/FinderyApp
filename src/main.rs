use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/devicemap.js")]
extern "C" {
    fn clearMapSearch();
}



#[function_component(App)]
fn app() -> Html {
    
    let thing = Callback::from(|_|{
        clearMapSearch();
    });

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();
    let onclick = Callback::from(move |_| {
        let input = input_ref.cast::<HtmlInputElement>().unwrap();
        let device_name = input.value();

        web_sys::console::log_1(&device_name.into());
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
                    <input ref={input_ref_outer.clone()} class="search-map-form" type="text" id="device-name" name="device-search-box" value="Thing"/>
                    <button class="search-map-form btn-hover color-5" id="search-device-btn" name="search-device-btn" {onclick}>{"Search"}</button>
                    <button class="search-map-form btn-hover color-5" id="clear-map-data" name="clear-map-data" onclick={thing}>{"Clear"}</button>    
                </div>
                <div class="search-results">
                    <div class="mapping search-block">
                        <iframe src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d193595.15830869428!2d-74.119763973046!3d40.69766374874431!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x89c24fa5d33f083b%3A0xc80b8f06e177fe62!2sNew%20York%20City%2C%20New%20York%2C%20USA!5e0!3m2!1sde!2sde!4v1573800931555!5m2!1sde!2sde" width="600" height="450" frameborder="0" style="border:0;"></iframe>
                    </div>
                    <div class="gps-data-output-table search-block">
                        <table>
                            <tr>
                                <th>{"Device Name:"}</th>
                                <td></td>
                            </tr>
                            <tr>
                                <th>{"Description:"}</th>
                                <td>{"Thing"}</td>
                            </tr>
                            <tr>
                                <th>{"Latitude:"}</th>
                                <td>{"Thing"}</td>
                            </tr>
                            <tr>
                                <th>{"Longitude:"}</th>
                                <td>{"Thing"}</td>
                            </tr>
                            <tr>
                                <th>{"Battery Life:"}</th>
                                <td>{"Thing"}</td>
                            </tr>
                            <tr>
                                <th>{"Time Stamp:"}</th>
                                <td>{"Thing"}</td>
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