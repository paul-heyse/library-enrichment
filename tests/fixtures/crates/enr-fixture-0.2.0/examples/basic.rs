//! The one example the fixture ships (tests/ACCEPTANCE_PLAN.md).

use enr_fixture::{Shape, Widget, describe, perimeter};

fn main() {
    let widget = Widget::new(4);
    println!(
        "{} has area {} and perimeter {}",
        describe(&widget),
        widget.area(),
        perimeter(&widget)
    );
}
