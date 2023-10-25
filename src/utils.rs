use crate::config::config;

pub fn get_random_pipe_v_offset() -> f32 {
    (config().window.height - config().borders.height) / 2. * (rand::random::<f32>() - 0.5)
}

pub fn total_pipes() -> &'static usize {
    static INSTANCE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

    let config = config();

    INSTANCE.get_or_init(|| {
        ((config.window.width + config.pipes.width / 2.) / config.pipes.interval).ceil() as usize
    })
}
