use clipboard_stream::ClipboardStream;
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut stream = ClipboardStream::new();

    while let Some(content) = stream.next().await {
        match content {
            Ok(v) => println!("{}", v),
            Err(e) => eprintln!("{}", e),
        }
    }
}
