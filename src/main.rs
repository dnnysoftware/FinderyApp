use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model{
        value: 0
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
            <div class="map-search-field">
                <form id="search-device-form" method="post" enctype="multipart/form-data" autocomplete="off">
                    <label class="search-map-form" for="device-name">{"Search Device:"}</label>
                    <input class="search-map-form" type="text" id="device-name" name="device-search-box"/>
                    <button class="search-map-form" id="search-device-btn" name="search-device-btn">{"Search"}</button>
                    <button class="search-map-form" id="clear-map-data" name="clear-map-data">{"Clear"}</button>
                </form>
            </div>
            <div class="map">
                <button {onclick}>{ "+1" }</button>
                <p>{state.value}</p>
            </div>
            <div class="gps-data-output-table">
                <table>
                    <tr>
                        <th>{"Device Name:"}</th>
                        <td>{"Thing"}</td>
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
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}