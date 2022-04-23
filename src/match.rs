#[derive(Debug)]
#[derive(PartialEq)]
enum Message {
    On,
    Off,
}

fn main() {
    let state = Message::Off;
    println!("Original state: {:#?}", state);
    let new_state = match Message::On {
        Message::On => true,
        Message::Off => false,
    };

    if state != new_state {
        match match new_state {
            true => println!("turn on light"),
            false => println!("turn off light"),
        } {
            Ok(_) => {
                println!("Match worked...")
            }
            Err(_) => {}
        }
    }

    println!("New state: {:#?}", new_state);
}

    /*
let new_state = match *m.message() {
                        LedMessage::On => true,
                        LedMessage::Off => false,
                        LedMessage::State(state) => state,
                        LedMessage::Toggle => !self.state,
                    };
                    if self.state != new_state {
                        match match new_state {
                            true => self.led.on(),
                            false => self.led.off(),
                        } {
                            Ok(_) => {
                                self.state = new_state;
                            }
                            Err(_) => {}
                        }
                    }
    */
