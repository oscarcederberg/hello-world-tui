use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("hello world!"))
        .title("hello_world_tui")
        .button("quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}