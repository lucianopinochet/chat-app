#![allow(non_snake_case)]


use dioxus::prelude::*;
#[derive(Debug)]
struct Message{
	id:u8,
	message:String
}
pub fn App(cx: Scope) -> Element{
	let messages = use_ref(cx, Vec::<Message>::new);
	let messages_lock = messages.read();
	let messages_rendered = messages_lock.iter().map(|message|{
		rsx!(Message(cx, message.message.clone(), message.id.clone()))
	});
	cx.render(rsx!(
		style {include_str!("./style.css")},
		div{
			class:"messages",
			messages_rendered,
		},
		SendBar(cx, messages)
	))
}

fn SendBar<'a>(cx: Scope<'a>, messages: &'a UseRef<Vec<Message>>) -> Element<'a>{
	let message = use_state(cx, || "".to_string());
	let mut next_id:&UseState<u8> = use_state(cx, || 0);
	cx.render(rsx!(
			form{
				onsubmit: move 	|e|{
					if let Some(value) = e.inner().values.get("send").clone(){
						messages.write().push(Message{
							id: next_id.get().clone(),
							message: value[0].clone()
						});
						message.set("".to_string())
					}
					
					println!("{:?}",messages.read());
					next_id += 1;
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
fn Message(cx: Scope, message: String, id:u8) -> Element{
	cx.render(rsx!(
		div{
			class:"message",
			key:"{id}",
			"{message}"
		}
	))
}