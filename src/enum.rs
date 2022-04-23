enum Thing {
    Empty,
    Something(i32),
    State(bool),
}

enum UartPins {
    Uart1_TxPa9,
    Uart1_RxPa10,
}

struct UartPair {
    pub rx: UartPins,
    pub tx: UartPins,
}

const UART1_PA_PINS: UartPair = UartPair { rx: UartPins::Uart1_RxPa10, tx: UartPins::Uart1_RxPa10 };

fn configure(up: UartPair:) {
    match up {
        UartPair(rx) => println!("Rx..."),
        UartPair(tx) => println!("Tx..."),
    }
}

fn main() {
    println!("Enum example");
    let t = Thing::Something(22);
    if let Thing::Something(value) = t {
        println!("t was: {}", value);
    }

    match Thing::State(true) {
        Thing::Something(value) => {
            println!("match Something: {}", value);
        }
        Thing::Empty => {
            println!("match Empty!");
        }
        Thing::State(x) => {
            println!("match State! {}", x);
        }
    }

    match t {
        _ => {
            println!("match anything!");
        }
    }

    //let uart1_pair = UartPair { rx: UartPins::Uart1_RxPa10, tx: UartPins::Uart1_RxPa10 };

}
