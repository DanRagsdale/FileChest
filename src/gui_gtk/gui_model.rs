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
//use file_chest::get_notes;

use std::fs;
use std::os::unix::prelude::DirEntryExt;

use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

pub struct AppModel {
	db: NotesDB,
    tasks: FactoryVecDeque<FileElement>,
	search_dir: String,
	show_hidden: bool,
	notes_buffer: gtk::TextBuffer,
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
						set_hexpand: true,
					    connect_activate[sender] => move |entry| {
					        let buffer = entry.buffer();
					        sender.input(AppMsg::AddDir(buffer.text()));
					        buffer.delete_text(0, None);
					    }
					},
					
					gtk::Button {
						set_icon_name: "edit-delete",
						set_margin_all: 12,

						connect_clicked[sender] => move |_| {
							sender.input(AppMsg::DeleteAll);
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
						}
					}
				},

				gtk::Box {
					set_orientation: gtk::Orientation::Horizontal,
					set_margin_all: 6,
					set_spacing: 6,

					gtk::ScrolledWindow {
						set_hscrollbar_policy: gtk::PolicyType::Never,
						set_min_content_height: 360,
						set_width_request: 200,
						set_vexpand: true,
                    
						#[local_ref]
						task_list_box -> gtk::ListBox {
							connect_row_selected[sender] => move |_self, opt| {
								if opt.is_some() {
									sender.input(AppMsg::SelectFile(opt.unwrap().index()));
								};
							},
						}
                	},

					gtk::ScrolledWindow {
						set_hexpand: true,
						set_vexpand: true,
						set_width_request: 300,
						
						gtk::TextView {
							set_buffer: Some(&model.notes_buffer),
						}
					}
				}
            }
        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::DeleteAll => {
                self.tasks.guard().clear();
				self.search_dir = String::from("");
            },
            AppMsg::AddDir(name) => {
				self.search_dir = name.clone();
				self.reload_dir();
            },
			AppMsg::SetShowHidden(do_show) => {
				self.show_hidden = do_show;
				self.reload_dir();
			},
			AppMsg::SelectFile(index) => {
				let fr =  &self.tasks.get(index as usize).unwrap().file;
				println!("Printing message for {:?}", fr.file_path);
				println!("This file has inode: {:?}", fr.inode);

				//let test_string = format!("Test! {}", index);
				self.notes_buffer.set_text(&self.db.get_note(fr).unwrap());
			}
        }
    }

    fn init(db: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel {
			db,
            tasks: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
			search_dir: String::from(""),
			show_hidden: false,
			notes_buffer: gtk::TextBuffer::builder().text("Hello World!").build(),
        };

        let task_list_box = model.tasks.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

impl AppModel {
	fn reload_dir(&mut self) {
		self.tasks.guard().clear();

		let paths = fs::read_dir(&self.search_dir).unwrap();
		let mut paths_vec: Vec<_> = vec![];
		for p in paths {
			let file = p.unwrap();
			if self.show_hidden || file.file_name().into_string().unwrap().as_bytes()[0] != '.' as u8 {
				paths_vec.push(file);
			}
		}
		paths_vec.sort_by_key(|dir| dir.path());

		for (_i, file) in paths_vec.iter().enumerate() {
			let file_path = file.path().clone();
			let inode = file.ino();
			let fr = FileRef { file_path, inode, };
			self.tasks.guard().push_back(fr);
		};
	}
}