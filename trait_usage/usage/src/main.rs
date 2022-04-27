use impl1::SomethingStruct as S1;
use impl2::SomethingStruct as S2;

use driver::Driver;

fn main() {
    // We can use or Driver with a impl1 or any other implemtation:
    let s = S1{};
    Driver::process(&s);

    // We can use or Driver with a impl2 or any other implemtation:
    let s = S2{};
    Driver::process(&s);
}
