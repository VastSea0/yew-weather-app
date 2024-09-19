use yew::prelude::*;
use yew::{html, Component, Context, Html};
use reqwest;
use reqwest::Error;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;


#[derive(Deserialize)]
struct WeatherResponse {
    location: Location,
    current: Current,
}

#[derive(Deserialize)]
struct Location {
    name: String,
}

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: String,
}

pub enum Msg {
    GetWeatherInfo(String),
    ReceiveWeatherInfo(Result<WeatherResponse, Error>),
    UpdateValue(String),
}

pub struct App {
    value: Option<WeatherResponse>,
    input_value: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: None,
            input_value: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetWeatherInfo(city) => {
                let ctx = _ctx.link().clone();
                spawn_local(async move {
                    let result = get_weather_info(&city).await;
                    ctx.send_message(Msg::ReceiveWeatherInfo(result));
                });
                true
            }
            Msg::ReceiveWeatherInfo(result) => {
                self.value = result.ok();
                true
            }
            Msg::UpdateValue(new_value) => {
                self.input_value = new_value;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let weather_info = self.value.as_ref().map(|weather| {
            html! {
                <div class={"info-card"} >
                    <h1>{ format!("Location: {}", weather.location.name) }</h1>
                    <p>{ format!("Temperature: {}°C", weather.current.temp_c) }</p>
                    <p>{ format!("Condition: {}", weather.current.condition.text) }</p>
                </div>
            }
        }).unwrap_or_else(|| html! { 
         
            <div class={"alert-card"} >
            <p>{"No weather info available."}</p> 
            
            </div>
          
        });

        html! {
            <div class={"app"} >
                <h1>{"Rust Yew Framework"}</h1>
                <h2>{"Weather App"}</h2>
                { weather_info.clone() }
                <input
                    type="text"
                    value={self.input_value.clone()}
                    oninput={_ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::UpdateValue(input.value())
                    })}
                />
             <p>{ format!("Input: {}", self.input_value) }</p>
            <button onclick={{
                let input_value = self.input_value.clone();
                _ctx.link().callback(move |_| Msg::GetWeatherInfo(input_value.clone()))
            }}>
                {"Get Weather Info"}
            </button>
            <br />
            <code>
            {"Create by Egehan KAHRAMAN using 🦀 with ❤️"}
            </code>
            </div>
        }
    } 
}

async fn get_weather_info(city: &str) -> Result<WeatherResponse, Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key=3d453ad4134449b0807210757241409&q={}&aqi=no",
        city
    );
    let response = reqwest::get(&url).await?;
    let weather: WeatherResponse = response.json().await?;
    Ok(weather)
}

fn main() {
    yew::Renderer::<App>::new().render();
}
