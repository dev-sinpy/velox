use velox::AppBuilder;

fn main() {
    let app = AppBuilder::from_config(include_str!("velox.conf.json").to_string()).build();
    app.run().unwrap();
}
