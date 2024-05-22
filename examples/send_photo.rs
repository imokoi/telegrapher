use core::bot::Bot;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    let photo_path = std::path::Path::new("examples/photo.jpg");
    let result = bot.send_photo("1393242628", photo_path).await;
    println!("{:?}", result);
}
