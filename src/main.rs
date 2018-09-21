extern crate gtk;
extern crate sourceview;

use std::path::PathBuf;
use std::fs;
use std::cell::RefCell;
use std::sync::Arc;

use gtk::prelude::*;
use gtk::{ResponseType, Box, CssProvider, Window, WindowType, Orientation, Menu, MenuBar, MenuItem, SeparatorMenuItem, ScrolledWindow, FileChooserDialog, FileChooserAction};
use sourceview::{View, ViewExt};


fn main() {
	if gtk::init().is_err() {
		println!("failed to load gtk");

		return;
	}

	let provider = CssProvider::new();
	if provider.load_from_data(b"* {font: 14pt \"Fira Mono\", monospace;}").is_err() {
		println!("failed to load css data");

		return;
	}

	let filename: Arc<RefCell<Option<PathBuf>>> = Arc::new(RefCell::new(None));

	let window = Window::new(WindowType::Toplevel);
	window.set_title("leafpad3");
	window.set_default_size(640, 480);

	let window_box = Box::new(Orientation::Vertical, 0);
	window.add(&window_box);

	//let window = Rc::new(window);

	let menu = MenuBar::new();
	window_box.add(&menu);
	
	let scrolled = ScrolledWindow::new(None, None);
	window_box.add(&scrolled);

	let text = View::new();
	text.set_show_line_numbers(true);
	text.set_hexpand(true);
	text.set_vexpand(true);
	text.set_monospace(true);
	text.set_tab_width(4);
	let context = text.get_style_context().unwrap();
	context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
	scrolled.add(&text);

	let file = MenuItem::new_with_label("File");
	menu.add(&file);

	let file_menu = Menu::new();
	file.set_submenu(&file_menu);

	let file_menu_new = MenuItem::new_with_label("New");
	// cloning for closure
	let w0 = window.clone();
	let b0 = text.get_buffer().unwrap().clone();
	let filename1 = filename.clone();
	file_menu_new.connect_activate(move |_| {
		b0.set_text("");
		filename1.replace(None);
		w0.set_title("leafpad3");
	});
	file_menu.add(&file_menu_new);

	let file_menu_open = MenuItem::new_with_label("Open");
	// cloning for closure
	let w1 = window.clone();
	let b1 = text.get_buffer().unwrap().clone();
	let filename2 = filename.clone();
	file_menu_open.connect_activate(move |_| {
		let dialog = FileChooserDialog::new(Some("Open File"), Some(&w1), FileChooserAction::Open);
		
		dialog.add_button("Cancel", ResponseType::Cancel.into());
		dialog.add_button("Open", ResponseType::Ok.into());

		if dialog.run() == ResponseType::Ok.into() {
			filename2.replace(dialog.get_filename());
			w1.set_title(&format!("leafpad3 - {:?}", filename2.borrow().clone().unwrap()));

			let file_result = fs::read_to_string(filename2.borrow().clone().unwrap());

			match file_result {
				Ok(output) => {
					b1.set_text(&output);
				},
				_ => println!("failed to open file")
			}
		}

		dialog.destroy();
	});
	file_menu.add(&file_menu_open);

	let file_menu_save = MenuItem::new_with_label("Save");
	// cloning for closure
	let b2 = text.get_buffer().unwrap().clone();
	let filename3 = filename.clone();
	file_menu_save.connect_activate(move |_| {
		if filename3.borrow().is_some() {
			let result = fs::write((*filename3.borrow()).clone().unwrap(), b2.get_text(&b2.get_start_iter(), &b2.get_end_iter(), false).unwrap());
		
			if result.is_err() {
				println!("failed to save file");
			}
		}
	});
	file_menu.add(&file_menu_save);

	let file_menu_save_as = MenuItem::new_with_label("Save As");
	// cloning for closure
	let w2 = window.clone();
	let b3 = text.get_buffer().unwrap().clone();
	let filename3 = filename.clone();
	file_menu_save_as.connect_activate(move |_| {
		let dialog = FileChooserDialog::new(Some("Save File"), Some(&w2), FileChooserAction::Save);
		
		dialog.add_button("Cancel", ResponseType::Cancel.into());
		dialog.add_button("Save", ResponseType::Ok.into());

		if dialog.run() == ResponseType::Ok.into() {
			filename3.replace(dialog.get_filename());
			w2.set_title(&format!("leafpad3 - {:?}", filename3.borrow()));

			let result = fs::write(filename3.borrow().clone().unwrap(), b3.get_text(&b3.get_start_iter(), &b3.get_end_iter(), false).unwrap());
			
			if result.is_err() {
				println!("failed to save file");
			}
		}

		dialog.destroy();
	});
	file_menu.add(&file_menu_save_as);

	let file_seperator = SeparatorMenuItem::new();
	file_menu.add(&file_seperator);

	let file_menu_quit = MenuItem::new_with_label("Quit");
	file_menu_quit.connect_activate(|_| {
		gtk::main_quit();
	});
	file_menu.add(&file_menu_quit);

	window.show_all();

	window.connect_delete_event(|_, _| {
		gtk::main_quit();
		Inhibit(false)
	});

	gtk::main();
}
