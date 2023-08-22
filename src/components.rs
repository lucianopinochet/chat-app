#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props, PartialEq )]
struct PropApp{
        name:&'static str
    }
pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
<<<<<<< HEAD
        div{
            
        }
    ))
=======
			style {include_str!("./style.css")}
			SendBar(cx)
		))

>>>>>>> 505254677c61e7c838107abc2b1b992fadb4bafa
}
fn SendBar(cx: Scope) -> Element{
	let message = use_state(cx, || "".to_string());

	cx.render(rsx!(
			form{
				onsubmit: move 	|e|{
					if let Some(value) = e.inner().values.get("send").clone(){
						println!("{:?}",value[0])
					}
				},
				prevent_default:""onsubmit,
				input{
					name:"send",
					class:"input-chat",
					value: "{message}",
					oninput: move |e| message.set(e.value.clone())
				}
				input{
					class:"input-submit",
					r#type:"submit"
				}
			}
	))
}