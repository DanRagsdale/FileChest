use crate::messages::*;
use crate::list_element::*;

use file_chest::FileRef;

use std::fs;
use std::os::unix::prelude::DirEntryExt;

use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

pub struct AppModel {
    tasks: FactoryVecDeque<Task>,
	search_dir: String,
}

#[relm4::component(pub)]
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
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}