use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, DropTarget, FileChooserDialog, Label, ResponseType,
};
use relm4::adw::prelude::AdwApplicationWindowExt;
use relm4::adw::{self, HeaderBar};
use relm4::gtk;
use relm4::gtk::prelude::GtkWindowExt;

fn build_ui(app: &Application) {
    // Function to handle file input and print the file name
    fn handle_file_input(file_name: &str) {
        println!("File received: {}", file_name);
    }

    // Create the "Open File..." button
    let button = Button::builder()
        .label("Open File...")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();
    button.add_css_class("pill");
    button.add_css_class("suggested-action");


    // Create the application window
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Drop Area with Button")
        .default_width(400)
        .default_height(300)
        .build();

    // Create a HeaderBar for the title bar
    let header_bar = HeaderBar::builder()
        // .title_widget(Some(&gtk::Label::new(Some("Drop Area with Button"))))
        .show_start_title_buttons(true) // Show buttons like minimize/maximize
        .show_end_title_buttons(true) // Show close button
        .build();

    // Create a label for instructions inside the drop area
    let drop_label = Label::new(Some("Drag and drop a file here"));

    // Create a box container to hold the drop area and button
    let container = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    container.append(&drop_label);
    container.append(&button);

    // Create the drop area with a DropTarget
    let drop_target = DropTarget::new(String::static_type(), gtk::gdk::DragAction::COPY);

    // Connect signal to handle files dropped in the drop area
    drop_target.connect_drop(move |_drop_target, value, _, _| {
        if let Ok(file_path) = value.get::<String>() {
            handle_file_input(&file_path);
        }
        false
    });

    // Attach drop target to the container
    container.add_controller(drop_target);

    // Create a main container to hold the header bar and content
    let main_container = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0) // No spacing between header bar and content
        .build();

    // Add the header bar and the main content container to the main container
    main_container.append(&header_bar);
    main_container.append(&container);

    // Set the main container as the content of the application window
    window.set_content(Some(&main_container));

    // Connect button "clicked" signal
    let window_clone = window.clone();
    button.connect_clicked(move |_| {
        // Create a FileChooserDialog
        let dialog = FileChooserDialog::builder()
            .title("Open File")
            .action(gtk::FileChooserAction::Open)
            .transient_for(&window_clone) // Set transient parent
            .modal(true) // Make it modal
            .build();

        // Add response buttons
        dialog.add_buttons(&[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Accept),
        ]);

        // Connect the response signal
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(file_path) = file.path() {
                        handle_file_input(file_path.to_str().unwrap());
                    }
                }
            }
            dialog.close();
        });

        // Show the dialog
        dialog.show();
    });

    // Present the window
    window.present();
}

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.example.GTKApp")
        .build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
