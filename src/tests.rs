use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    FmtSubscriber,
};

#[ctor::ctor]
fn before_each_unit_test() {
    let log_subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy()
                .add_directive("hyper=info".parse().unwrap()),
        )
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(log_subscriber)
        .expect("setting default subscriber failed");
}
