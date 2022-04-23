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
    type BoardConfig: Default;

    fn new(peripherals: Self::Peripherals) -> Self;
    fn new_with_config(peripherals: Self::Peripherals, config: BoardConfig) -> Self;
}

struct DiscoBoard {}

struct Pers {}
struct Conf {}

impl Board for DiscoBoard {
    type Peripherals = Pers;
    type BoardConfig = Conf;

    fn new(peripherals: Self::Peripherals) -> Self {
        DiscoBoard {}
    }

    fn new(peripherals: Self::Peripherals, config: Self::BoardConfig>) -> Self {
        DiscoBoard {}
    }
}

fn main() {
    let p = Pers{};
    let c = Some(Conf{});
    let ds = DiscoBoard::new(p, c);
}
