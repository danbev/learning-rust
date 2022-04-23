#[non_exhaustive]
pub struct Config {
    pub window_width: u16,
    pub window_height: u16,
}

fn main() {
    let config = Config { window_width: 640, window_height: 480 };

    let Config { window_width, window_height, .. } = config;
}
