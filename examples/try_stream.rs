use clipboard_stream::{ClipboadEventListener, Kind};
use futures::stream::TryStreamExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut event_lisener = ClipboadEventListener::spawn();
    let mut stream = event_lisener.new_stream(Kind::Utf8String, 32)?;

    let future = async move {
        loop {
            if let Ok(Some(body)) = stream.try_next().await {
                println!("clipboard updated: {:?}", body);
            }
        }
    };

    let _ = futures::executor::block_on(future);
    Ok(())
}
