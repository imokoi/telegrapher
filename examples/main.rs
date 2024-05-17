use core::bot::Bot;

#[tokio::main]
async fn main() {
    let bot = Bot::new("6616659571:AAEr0TdwPXBnvHQl_VJj5Z6wh-p3uUDNbOw");
    bot.start().await.unwrap();
}
