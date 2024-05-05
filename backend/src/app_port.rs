pub(crate) fn get_app_port() -> u16 {
    let port = std::env::var("APP_PORT").ok();
    let port = if let Some(port) = port {
        match port.parse() {
            Ok(port) => Some(port),
            Err(err) => {
                log::error!("Invalid port was given as env: '{}', {err}", port);
                None
            }
        }
    } else {
        log::info!("No port is set as env: default will be used");
        None
    };
    let port = port.unwrap_or(20103);
    log::info!("Port to use: {port}");
    port
}
