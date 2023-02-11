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

/* This GUI is an evolution of the "To Do" example on the relm4 github
* https://github.com/Relm4/Relm4/blob/main/examples/to_do.rs
*/

use std::fs;
use std::path::PathBuf;
use std::os::unix::prelude::DirEntryExt;

use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

#[derive(Debug)]
struct FileRef {
	file_path: PathBuf,
	inode: u64,
}

#[derive(Debug)]
struct Task {
	file: FileRef,
    completed: bool,
}

#[derive(Debug)]
enum TaskInput {
    Toggle(bool),
}

#[derive(Debug)]
enum TaskOutput {
    Delete(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for Task {
    type Init = FileRef;
    type Input = TaskInput;
    type Output = TaskOutput;
    type CommandOutput = ();
    type ParentInput = AppMsg;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            //gtk::CheckButton {
            //    set_active: false,
            //    set_margin_all: 12,
            //    connect_toggled[sender] => move |checkbox| {
            //        sender.input(TaskInput::Toggle(checkbox.is_active()));
            //    }
            //},

            #[name(label)]
            gtk::Label {
                set_label: &self.file.file_path.file_name().unwrap().to_str().unwrap(),
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                set_margin_all: 12,
            },

            //gtk::Button {
            //    set_icon_name: "edit-delete",
            //    set_margin_all: 12,

            //    connect_clicked[sender, index] => move |_| {
            //        sender.output(TaskOutput::Delete(index.clone()));
            //    }
            //}
        }
    }

    fn pre_view() {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn output_to_parent_input(output: Self::Output) -> Option<AppMsg> {
        Some(match output {
            TaskOutput::Delete(index) => AppMsg::DeleteEntry(index),
        })
    }

    fn init_model(file: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
			file,
            completed: false,
        }
    }
}

#[derive(Debug)]
enum AppMsg {
    DeleteEntry(DynamicIndex),
	DeleteAll,
    AddDir(String),
	SelectFile(i32),
}

struct AppModel {
    tasks: FactoryVecDeque<Task>,
	search_dir: String,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
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
				},

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 360,
                    set_vexpand: true,

                    #[local_ref]
                    task_list_box -> gtk::ListBox {
						connect_row_selected[sender] => move |_self, opt| {
							if opt.is_some() {
								sender.input(AppMsg::SelectFile(opt.unwrap().index()));
							};
						},
					}
                }
            }

        }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::DeleteEntry(index) => {
                self.tasks.guard().remove(index.current_index());
            },
            AppMsg::DeleteAll => {
                self.tasks.guard().clear();
            },
            AppMsg::AddDir(name) => {
				self.search_dir = name.clone();
				self.tasks.guard().clear();

				let paths = fs::read_dir(&self.search_dir).unwrap();
				let mut paths_vec: Vec<_> = vec![];
				for p in paths {
					let file = p.unwrap();
					//if self.show_hidden || file.file_name().into_string().unwrap().as_bytes()[0] != '.' as u8 {
						paths_vec.push(file);
					//}
				}
				paths_vec.sort_by_key(|dir| dir.path());

				for (_i, file) in paths_vec.iter().enumerate() {
					let file_path = file.path().clone();
					let inode = file.ino();
					let fr = FileRef { file_path, inode, };
					self.tasks.guard().push_back(fr);
				};
            },
			AppMsg::SelectFile(index) => {
				let fr =  &self.tasks.get(index as usize).unwrap().file;
				println!("Printing message for {:?}", fr.file_path);
				println!("This file has inode: {:?}", fr.inode);
			}
        }
    }

    fn init(_: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel {
            tasks: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
			search_dir: String::from(""),
        };

        let task_list_box = model.tasks.widget();
		//task_list_box.connect_row_selected(|_self, _opt| {
		//	println!("Test, Test, Test");
		//});
			
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("com.danielragsdale.file_chest");
    app.run::<AppModel>(());
}