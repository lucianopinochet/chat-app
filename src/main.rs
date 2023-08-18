#![allow(non_snake_case)]

mod components;
use components::App;

//use dioxus::prelude::*;


fn main() {
    
    dioxus_desktop::launch(App);
}

