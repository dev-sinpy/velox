use velox::utils::is_dev;
use velox::utils::ContentType;
use velox::AppBuilder;

fn main() {
    let window_content = if is_dev() {
        ContentType::Url("https://google.com".to_string())
    } else {
        ContentType::File("assets/index.html".to_string())
    };
    let app = AppBuilder::from_config(include_str!("./velox.conf.json").to_string())
        .load(window_content)
        .build();
    app.run().unwrap();
}
