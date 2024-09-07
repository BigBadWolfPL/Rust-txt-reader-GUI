use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, TextView, ScrolledWindow, Box};
use gtk::Adjustment;
use std::fs::File;
use std::io::Read;

fn main() {
    // Inicjalizacja GTK
    let application = Application::new(
        Some("com.example.fileviewer"),
        Default::default(),
    );

    application.connect_activate(|app| {
        // Tworzenie głównego okna
        let window = ApplicationWindow::new(app);
        window.set_title("File Viewer");
        window.set_default_size(600, 400);

        // Tworzenie kontenera
        let vbox = Box::new(gtk::Orientation::Vertical, 5);

        // Tworzenie Entry do wprowadzania ścieżki
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Wprowadź ścieżkę do pliku..."));

        // Tworzenie przycisku
        let button = Button::with_label("Odczytaj plik");

        // Tworzenie TextView do wyświetlania zawartości
        let text_view = TextView::new();
        let scrolled_window = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrolled_window.add(&text_view);

        // Dodawanie widżetów do kontenera
        vbox.pack_start(&entry, false, false, 0);
        vbox.pack_start(&button, false, false, 0);
        vbox.pack_start(&scrolled_window, true, true, 0);

        window.add(&vbox);

        // Obsługa kliknięcia przycisku
        button.connect_clicked(move |_| {
            let path = entry.text().to_string();
            let mut file_content = String::new();
            if let Ok(mut file) = File::open(&path) {
                if let Ok(_) = file.read_to_string(&mut file_content) {
                    let buffer = text_view.buffer().expect("Brak bufora");
                    buffer.set_text(&file_content);
                } else {
                    let buffer = text_view.buffer().expect("Brak bufora");
                    buffer.set_text("Błąd odczytu pliku.");
                }
            } else {
                let buffer = text_view.buffer().expect("Brak bufora");
                buffer.set_text("Nie można otworzyć pliku.");
            }
        });

        // Wyświetlenie okna
        window.show_all();
    });

    application.run();
}
