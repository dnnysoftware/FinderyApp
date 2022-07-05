use yew::prelude::*;

struct Model {
    value: i64
}

struct Thing {
    device: String
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model{
        value: 0
    });

    let state_device = use_state(|| Thing{
        device: String::from("Wallet")
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

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
                    <form id="search-device-form" method="post" enctype="multipart/form-data" autocomplete="off">
                        <label class="search-map-form" for="device-name">{"SEARCH DEVICE:"}</label>
                        <input class="search-map-form" type="text" id="device-name" name="device-search-box"/>
                        <button class="search-map-form btn-hover color-5" id="search-device-btn" name="search-device-btn">{"Search"}</button>
                        <button class="search-map-form btn-hover color-5" id="clear-map-data" name="clear-map-data">{"Clear"}</button>
                    </form>
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
                <button {onclick}>{ "+1" }</button>
                <p>{state.value}</p>
            </div>
        </div>
       </>
    }
}

fn main() {
    yew::start_app::<App>();
}