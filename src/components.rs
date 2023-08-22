#![allow(non_snake_case)]

use dioxus::prelude::*;
#[derive(Props, PartialEq )]
struct PropApp{
        name:&'static str
    }
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!{
        Greet{name: "Luciano"}
    })
}
fn Greet(cx:Scope<PropApp>) -> Element{
    cx.render(rsx!(
        div{
            
        }
    ))
}
