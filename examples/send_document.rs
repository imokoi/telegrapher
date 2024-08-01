use core::{bot::Bot, models::sticker::FileUpload};
use telegrapher::params::media_params::SendDocumentParamsBuilder;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw", 1);
    let send_document_params = SendDocumentParamsBuilder::default()
        .chat_id(1393242628)
        .document(FileUpload::String("https://cf-ipfs.com/ipfs/QmP3R6KfKKH1C4gBSnGg7kWgPK4gnMxs74bR23QrTUezow?random=171988623".to_string()))
        .build()
        .unwrap();
    let result = bot.send_document(&send_document_params).await;
    println!("{:?}", result);
}
