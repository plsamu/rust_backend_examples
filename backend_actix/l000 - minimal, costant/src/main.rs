// https://www.youtube.com/c/GenusvProgramming/playlists

mod constant;
mod server;

#[tokio::main]
async fn main() {
    server::main().await.unwrap();
}