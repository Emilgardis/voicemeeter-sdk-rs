use voicemeeter::VoicemeeterRemote;

fn main() {
    run();
    VoicemeeterRemote::new().expect_err("we've logged out so shouldn't login again");
}
fn run() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();
    let vmr = VoicemeeterRemote::new().unwrap();
    let vmr2 = VoicemeeterRemote::new().unwrap();
    drop(vmr);
    drop(vmr2);
}
