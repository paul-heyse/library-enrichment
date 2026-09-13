//! The one example the fixture ships (tests/ACCEPTANCE_PLAN.md).

use enr_fixture::{Shape, Widget, describe};

fn main() {
    let widget = Widget::new(4);
    println!("{} has area {}", describe(&widget), widget.area());
}
