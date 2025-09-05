use clipboard_stream::event_listener::ClipboardEventListener;
use futures::StreamExt;


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let event=  ClipboardEventListener::new();
    let mut s = event.new_stream(clipboard_stream::body::Kind::Utf8).unwrap();

    while let Some(v) = s.next().await {
        println!("{:?}", v);
    }

}
