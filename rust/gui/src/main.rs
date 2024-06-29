use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, CheckButton,
    ComboBoxText, Entry, Label, Orientation, Alignment,
};

fn main() {
    let application = Application::new(
        Some("com.example.helloworld"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Hello World");
        window.set_default_size(400, 200);

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        vbox.set_size_request(350, -1);  // Установка ширины vbox

        let label = Label::new(Some("Hello World"));
        vbox.pack_start(&label, false, false, 0);

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter text here"));
        vbox.pack_start(&entry, false, false, 0);

        let combo_box = ComboBoxText::new();
        combo_box.append_text("Option 1");
        combo_box.append_text("Option 2");
        combo_box.append_text("Option 3");
        combo_box.set_active(Some(0));
        vbox.pack_start(&combo_box, false, false, 0);

        let check_button = CheckButton::with_label("Check me!");
        vbox.pack_start(&check_button, false, false, 0);

        let alignment = Alignment::new(0.5, 0.5, 0.0, 0.0);  // Центрирование содержимого
        alignment.add(&vbox);

        window.add(&alignment);  // Добавление alignment в окно

        window.show_all();
    });

    application.run();
}
