#![allow(non_snake_case)]


use dioxus::prelude::*;
use dioxus_router::prelude::*;
#[derive(Debug)]
struct Message{
	id:	u8,
	message:	String
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
	#[route("/", Login)]
		Login{},
	#[route("/:name", ChatRoom)]
		ChatRoom{ 
			name:String 
		},
}

pub fn App(cx: Scope) -> Element{
	render!{
		Router::<Route>{}
	}
}

fn Login<'a>(cx: Scope<'a>) -> Element<'a>{
	let name = use_state(cx, || "".to_string());
	render!{
		div{
			class:"login",
			form{	
				prevent_default:"onsubmit",
				input{
					name:"name",
					value:"{name}",
					oninput: move |e| name.set(e.value.clone())
				},
				Link{
					to: Route::ChatRoom {
						name:name.get().clone()
					},
					input{
						r#type:"submit"	
					},
				}
			},
		}	
	}
}
fn ChatRoom<'a>(cx: Scope<'a>, name:String) -> Element<'a>{
	println!("{name}");
	let messages = use_ref(cx, Vec::<Message>::new);
	let messages_lock = messages.read();
	let messages_rendered = messages_lock.iter().map(|message|{
		render!(Message(cx, message.message.clone(), message.id.clone()))
	});
	render!{
		style {include_str!("./style.css")},
		div{
			class:"messages",
			messages_rendered,
		},
		SendBar(cx, messages)
	}
}

fn SendBar<'a>(cx: Scope<'a>, messages: &'a UseRef<Vec<Message>>) -> Element<'a>{
	let message = use_state(cx, || "".to_string());
	let mut next_id:&UseState<u8> = use_state(cx, || 0);
	render!{
			form{
				onsubmit: move 	|e|{
					if let Some(value) = e.inner().values.get("send").clone(){
						messages.write().push(Message{
							id: next_id.get().clone(),
							message: value[0].clone()
						});
						message.set("".to_string())
					}
					// println!("{:?}",messages.read());
					next_id += 1;
				},
				prevent_default:"onsubmit",
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
		}
}
fn Message(cx: Scope, message: String, id:u8) -> Element{
	render!{
		div{
			class:"message",
			key:"{id}",
			"{message}"
		}
	}
}