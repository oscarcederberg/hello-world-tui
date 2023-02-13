use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("hello world!"))
        .title("hello_world_tui")
        .button("next", show_next));

    // Starts the event loop.
    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("next screen?")
        .title("next")
        .button("yes", show_complex)
        .button("pop-up", |s| s.add_layer(Dialog::info("pop-up!"))));
}

fn show_complex(s: &mut Cursive) {
    let mut select = SelectView::<String>::new()
        .on_submit(on_submit)
        .fixed_size((10, 5));
    let buttons = LinearLayout::vertical()
        .child(Button::new("button", |s| ()))
        .child(DummyView)
        .child(Button::new("quit", Cursive::quit));

    select.get_inner_mut().add_all_str(["one", "two"].into_iter());

    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(buttons))
        .title("complex"));
}

fn on_submit(s: &mut Cursive, selected: &str) {
    s.add_layer(Dialog::text(format!("you chose {}", selected))
        .title(format!("{}", selected))
        .button("OK", Cursive::quit));
}