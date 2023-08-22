#![allow(non_snake_case)]

mod components;

use components::App;
	// use dioxus_router::prelude::*; 
// use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

// #[derive(Routable, Clone)]
// enum Route{
// 	#[route("/login")]
// 	App{}
// }
#[tokio::main]
async fn main() {
  dioxus_desktop::launch_cfg(
		App, 
		Config::default()
		.with_window(WindowBuilder::new()
		.with_resizable(true)
		.with_title("Chat App")
		)
	);
}

