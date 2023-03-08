/* Copyright (c) 2023 Daniel Ragsdale <DanJeffRags@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program; if not, write to the Free Software Foundation, Inc., 59 Temple
 * Place, Suite 330, Boston, MA  02111-1307  USA
 */

use crate::messages::*;
use crate::file_element::*;

use file_chest::{FileRef, NotesDB};

use std::fs;
use std::process::Command;

use gtk::prelude::*;

use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

pub struct AppModel {
	db: NotesDB,
    file_elements: FactoryVecDeque<FileElement>,
	search_dir: String,
	show_hidden: bool,
	dir_entry_buffer: gtk::EntryBuffer,
	tag_entry_buffer: gtk::EntryBuffer,
	notes_buffer: gtk::TextBuffer,
	current_file: Option<FileRef>,
	p_menu: gtk::PopoverMenu,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = NotesDB;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_width_request: 360,
            set_title: Some("To-Do"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 6,
				
				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_margin_all: 6,
					set_spacing: 6,

					gtk::Entry {
						set_placeholder_text: Some("Enter a directory"),
						set_buffer: &model.dir_entry_buffer,

						set_hexpand: true,
					    connect_activate[sender] => move |entry| {
					        let buffer = entry.buffer();
					        sender.input(AppMsg::SetDir(buffer.text()));
					        //buffer.delete_text(0, None);
					    }
					},

					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_margin_all: 2,
						set_spacing: 2,

						gtk::Switch {
							set_active: model.show_hidden,
							connect_state_set[sender] => move |_self, do_show| {
								sender.input(AppMsg::SetShowHidden(do_show));
								gtk::Inhibit(false)
							},
						},

						gtk::Label {
							set_text: "Show Hidden Files",
						},
						
					}
				},

				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_margin_all: 6,
					set_spacing: 6,

					#[local_ref]
					click_box -> gtk::Box{
						#[local_ref]
						p_menu -> gtk::PopoverMenu {},

						gtk::ScrolledWindow {
							set_hscrollbar_policy: gtk::PolicyType::Never,
							set_min_content_height: 360,
							set_width_request: 300,
							set_hexpand: true,
							set_vexpand: true,
						
							#[local_ref]
							files_list -> gtk::ListBox {
								set_activate_on_single_click: false,

								connect_row_selected[sender] => move |_self, opt| {
									if opt.is_some() {
										sender.input(AppMsg::SelectFile(opt.unwrap().index()));
									};
								},

								connect_row_activated[sender] => move |_self, opt| {
									sender.input(AppMsg::SelectFile(opt.index()));
									sender.input(AppMsg::SetDirFromSelected)
								},
							},
						},
					},

					gtk::Box {
						set_orientation: gtk::Orientation::Vertical,
						set_margin_all: 6,
						set_spacing: 6,

						gtk::ScrolledWindow {
							set_hexpand: true,
							set_vexpand: true,
							set_width_request: 300,

							gtk::TextView {
								set_vexpand: true,
								set_buffer: Some(&model.notes_buffer),
							},
						},

						gtk::Button {
							//set_icon_name: "edit-delete",
							set_label: "Submit Note",
							set_margin_all: 12,

							connect_clicked[sender] => move |_| {
								sender.input(AppMsg::SubmitNote);
							}
						},
					
						gtk::Entry {
							set_placeholder_text: Some("Enter tags"),
							set_buffer: &model.tag_entry_buffer,

							set_hexpand: true,
							connect_activate[sender] => move |entry| {
					        	let buffer = entry.buffer();
					        	sender.input(AppMsg::SubmitTags(buffer.text()));
							},
						},
					}
				}
            }
        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::SetDir(name) => {
				let len = name.len();
				if len > 4 && &name[0..4] == "tag:" {
					if let Ok(files) = self.db.get_files_by_tag((&name[4..len]).trim()) {
						self.file_elements.guard().clear();
						for fr in files {
							self.file_elements.guard().push_back(fr);
						}
					}
				} else {
					self.search_dir = name;
					self.reload_dir();
				}
            },
			AppMsg::SetDirFromSelected => {
				if let Some(file) = &self.current_file {
					self.search_dir = file.file_path.to_string_lossy().to_string();
					self.reload_dir();
					self.dir_entry_buffer.set_text(&self.search_dir);
				}
			},
			AppMsg::SetShowHidden(do_show) => {
				self.show_hidden = do_show;
				self.reload_dir();
			},
			AppMsg::SelectFile(index) => {
				let fr = self.get_fileref_by_index(index as usize).unwrap(); 

				//let test_string = format!("Test! {}", index);
				match self.db.get_note(&fr) {
					Ok(note) => {
						self.notes_buffer.set_text(&note);
					},
					Err(_) => {
						self.notes_buffer.set_text("Enter a new note!");
					},
				};
				
				match self.db.get_tags(&fr) {
					Ok(tags) => {
						self.tag_entry_buffer.set_text(&tags.join(", "));
					},
					Err(_) => {},
				};

				self.current_file = Some(fr.clone());
			},
			AppMsg::SubmitNote => {
				if let Some(file) = &self.current_file {
					let start = self.notes_buffer.start_iter();
					let end = self.notes_buffer.end_iter();
					if let Err(e) = self.db.set_note(file, self.notes_buffer.text(&start, &end, true).as_ref()) {
						eprintln!("Error submitting note {e}");
					}
				}
			},
			AppMsg::SubmitTags(tag_string) => {
				if self.current_file.is_some() {
					let tags = tag_string.split(",").map(|t| t.trim()).collect();
					
					if let Some(file) = &self.current_file {
						if let Err(e) = self.db.set_tags(file, tags) {
							eprintln!("Error submitting tags {e}");
						}
					}
				}
			},
			AppMsg::ShowFileContext(x, y) => {
				if let Some(_) = &self.current_file {
					self.p_menu.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x as i32, y as i32, 100, 0)));
					self.p_menu.popup();
				}
			},
			AppMsg::OpenCurrentFile(open_type) => {
				if let Some(cur_file) = &self.current_file {
					match open_type {
						OpenType::OpenFile => {
							let path = cur_file.file_path.to_str().expect("Could not unwrap file path");
							Command::new("xdg-open").arg(path).output().expect("Failed to open file");
						},
						OpenType::OpenParent => {
							if let Some(parent_path) = cur_file.file_path.parent() {
								let path = parent_path.to_str().expect("Could not unwrap file path");
								Command::new("xdg-open").arg(path).output().expect("Failed to open file");
							}
						}
					}
				}
			},
        }
    }

    fn init(db: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
		let menu_list = gtk::gio::Menu::new();
		menu_list.append(Some("Open File"), Some("win.action_open"));
		menu_list.append(Some("Open Parent Directory"), Some("win.action_view"));

		let p_menu = gtk::PopoverMenu::builder()
			.menu_model(&menu_list)
			.has_arrow(false)
			.width_request(100).build();

        let model = AppModel {
			db,
            file_elements: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
			search_dir: String::from(""),
			show_hidden: false,
			dir_entry_buffer: gtk::EntryBuffer::new(Some("")),
			tag_entry_buffer: gtk::EntryBuffer::new(Some("")),
			notes_buffer: gtk::TextBuffer::builder().text("Hello World!").build(),
			current_file: None,
			p_menu: p_menu.clone(),
        };

		let open_sender = sender.clone();
		let action_open = gtk::gio::SimpleAction::new("action_open", None);
		action_open.connect_activate(move |_, _| {
			open_sender.input(AppMsg::OpenCurrentFile(OpenType::OpenFile));
		});
		root.add_action(&action_open);
		
		let view_sender = sender.clone();
		let action_view = gtk::gio::SimpleAction::new("action_view", None);
		action_view.connect_activate(move |_, _| {
			view_sender.input(AppMsg::OpenCurrentFile(OpenType::OpenParent));
		});
		root.add_action(&action_view);



        let files_list = model.file_elements.widget();
		let click_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let widgets = view_output!();

		// Right Click handler for opening and viewing files
		let gesture = gtk::GestureClick::new();
		gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);
		gesture.connect_pressed(move |gesture, _n, x, y| {
		    gesture.set_state(gtk::EventSequenceState::Claimed);
			sender.input(AppMsg::ShowFileContext(x, y));
		});
		click_box.add_controller(&gesture);


		ComponentParts { model, widgets }
    }
}

impl AppModel {
	fn get_fileref_by_index(&self, index: usize) -> Option<FileRef> {
		if let Some(fe) = self.file_elements.get(index) {
			return Some(fe.file.clone());
		};

		None
	}

	fn reload_dir(&mut self) {
		self.file_elements.guard().clear();

		if let Ok(paths) = fs::read_dir(&self.search_dir)
		{
			let mut paths_vec: Vec<std::fs::DirEntry> = paths
				.map(|p| p.unwrap())
				.filter(|f| self.show_hidden || f.file_name().into_string().unwrap().as_bytes()[0] != '.' as u8 )
				.collect();
			//let mut paths_vec: Vec<_> = vec![];
			//for p in paths {
			//	let file = p.unwrap();
			//	if self.show_hidden || f.file_name().into_string().unwrap().as_bytes()[0] != '.' as u8 {
			//		paths_vec.push(file);
			//	}
			//}
			paths_vec.sort_by_key(|dir| dir.path());

			for (_i, file) in paths_vec.iter().enumerate() {
				let fr = FileRef::from_direntry(&file).expect("Tried to create invalid FileRef");
				self.file_elements.guard().push_back(fr);
			};
		}
	}
}
