// use demo::logger;
// use demo::ENV;
// use slog::{info, warn};

//FIXME: rm env_logger
// slog is the log package

//TODO: find a way to supper log rotate

fn main() -> std::io::Result<()> {
    demo::init();

    // let url = &ENV.database.url;

    // info!(&logger, "env"; "ENV.database"=> format!("{:?}", ENV.database));
    // warn!(&logger,"111"; "url" => url);
    demo::run()
}
