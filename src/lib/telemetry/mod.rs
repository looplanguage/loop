#[cfg(debug_assertions)]
fn get_build() -> String {
    "development".to_string()
}

#[cfg(not(debug_assertions))]
fn get_build() -> String {
    "release".to_string()
}

pub fn enable_telemetry() {
    let _guard = sentry::init((
        "https://d071f32c72f44690a1a7f9821cd15ace@o1037493.ingest.sentry.io/6005454",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(get_build().into()),
            ..Default::default()
        },
    ));
}
