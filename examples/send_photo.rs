use core::{
    bot::Bot,
    models::sticker::{FileUpload, InputFile},
    params::media_params::SendPhotoParamsBuilder,
};

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw", 1);
    let photo_path = std::path::Path::new("examples/photo.jpg");
    // let result = bot.send_photo("1393242628", photo_path).await;
    let send_photo_params = SendPhotoParamsBuilder::default()
        .chat_id("1393242628")
        .photo(FileUpload::InputFile(InputFile {
            path: photo_path.to_path_buf(),
        }))
        .build()
        .unwrap();
    // let send_photo_params = SendPhotoParamsBuilder::default()
    //     .chat_id("1393242628")
    //     .photo(FileUpload::String(
    //         "AgACAgUAAxkDAAPHZk1s-X8W_vDHUz2rCr30jHUthj8AAk69MRu6t3FWk1IgCPWm39oBAAMCAAN4AAM1BA"
    //             .to_string(),
    //     ))
    //     .build()
    //     .unwrap();
    let result = bot.send_photo(&send_photo_params).await;
    println!("{:?}", result);
}
