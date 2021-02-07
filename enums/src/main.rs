enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

fn inspect(web_event: WebEvent) {
    match web_event {
        WebEvent::PageLoad => println!("page got loaded"),
        WebEvent::PageUnload => println!("page got unloaded"),
        WebEvent::KeyPress(c) => println!("you pressed key {}", c),
        WebEvent::Paste(s) => println!("Copy and paste is what developers do {}", s),
        WebEvent::Click { x, y } => println!("{},{}", x, y),
    }
}

fn main() {
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);
    inspect(WebEvent::KeyPress('a'));
    inspect(WebEvent::Paste("let foo = 1;".to_string()));
    inspect(WebEvent::Click { x: 30, y: 10 });
}
