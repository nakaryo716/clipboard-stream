use clipboard_stream::ClipboardStream;
use futures::stream::TryStreamExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = ClipboardStream::new();

    let future = async move {
        loop {
            if let Ok(Some(item)) = stream.try_next().await {
                println!("clipboard updated: {}", item);

                // if the clipboard item is "stop", system will shutdown
                if item.as_str() == "stop" {
                    break;
                }
            }
        }
    };

    let _ = futures::executor::block_on(future);
    Ok(())
}
