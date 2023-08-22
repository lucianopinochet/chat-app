#![allow(non_snake_case)]

use dioxus::prelude::*;

// #[derive(Props, PartialEq )]
// struct PropApp{
//         name:&'static str
//     }

pub fn SendBar(cx: Scope) -> Element{
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
pub fn Messages<'a>(cx:Scope, messages: Vec<&'a str>) -> Element::<'a>{
	cx.render(rsx!(
		for message in messages{
			Message(cx, message)
		}
	))
}
fn Message<'a>(cx: Scope, message: &'a str) -> Element::<'a>{
	cx.render(rsx!(
		div{
			class:"message"
		}
	))
}