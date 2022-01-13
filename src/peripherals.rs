pub mod peripherals {
    pub struct PA1 {
        _private: ()
    }

    pub struct PA2 {
        _private: ()
    }
}

pub struct Peripherals {
    PA1: peripherals::PA1,
    PA2: peripherals::PA2,
}

pub trait Board : Sized {
    type Peripherals;
    type Config;

    fn new(peripherals: Self::Peripherals, config: Option<Self::Config>) -> Self;
}

struct DiscoBoard {}

struct Pers {}
struct Conf {}

impl Board for DiscoBoard {
    type Peripherals = Pers;
    type Config = Conf;

    fn new(peripherals: Self::Peripherals, config: Option<Self::Config>) -> Self {
        if config.is_some() {

        } else {

        }
        DiscoBoard {}
    }
}

fn main() {
    let p = Pers{};
    let c = Some(Conf{});
    let ds = DiscoBoard::new(p, c);
}
