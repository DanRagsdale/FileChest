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

use std::fs;

use file_chest::get_notes;

use iced::theme::Theme;
use iced::widget::{text, button, toggler, column, container, horizontal_rule, row, scrollable, text_input, radio,};
use iced::{Alignment, Element, Length, Sandbox, Settings};

/*
* This gui is evolved from the "styling" example on the iced github.
* https://github.com/iced-rs/iced/tree/master/examples/styling
*/

pub fn main() -> iced::Result {
    ChestApp::run(Settings::default())
}

#[derive(Default)]
struct ChestApp {
    theme: Theme,
    input_value: String,
	search_dir: String,
	selected_file: usize,
	show_hidden: bool,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DirSubmit,
	DoShowHidden(bool),
	FileSelected(usize),
}

impl Sandbox for ChestApp {
    type Message = Message;

    fn new() -> Self {
        ChestApp {theme: Theme::Light, input_value: String::from(""), search_dir: String::from("./"), selected_file: 0, show_hidden: false}
    }

    fn title(&self) -> String {
        String::from("File Chest")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::DirSubmit => {
				self.search_dir = self.input_value.clone();
				self.selected_file = 0;
			},
			Message::DoShowHidden(b) => {
				self.show_hidden = b;
				self. selected_file = 0;
			},
			Message::FileSelected(i) => self.selected_file = i,
        }
    }

    fn view(&self) -> Element<Message> {

        let text_input = text_input(
            "Enter a directory",
            &self.input_value,
            Message::InputChanged,
        ).on_submit(Message::DirSubmit)
        .padding(10)
        .size(20);

        let button = button("Submit")
            .padding(10)
            .on_press(Message::DirSubmit);

        let toggler_hidden = toggler(
            String::from("Show Hidden Files"),
            self.show_hidden,
            Message::DoShowHidden,
        )
        .width(Length::Shrink)
        .spacing(10);

		let mut files = column![].spacing(10).width(Length::Fill);

		let paths = fs::read_dir(&self.search_dir).unwrap();
    	let mut paths_vec: Vec<_> = vec![];
		for p in paths {
			let file = p.unwrap();
			if self.show_hidden || file.file_name().into_string().unwrap().as_bytes()[0] != '.' as u8 {
				paths_vec.push(file);
			}
		}
		paths_vec.sort_by_key(|dir| dir.path());

		for (i, path) in paths_vec.iter().enumerate() {
			files = files.push(radio(
				path.file_name().into_string().unwrap(),
				i,
				Some(self.selected_file),
				Message::FileSelected,
			));
		};

		let files_disp = scrollable(
			files,
		);

		let mut file_notes = text("");
		if !paths_vec.is_empty() {
			file_notes = text(get_notes(&paths_vec[self.selected_file].path().display().to_string()).expect("Selected file should exist"))
		}
        let annotation_disp = scrollable(
			file_notes
        );

        let content = column![
            row![text_input, button, toggler_hidden].spacing(10).align_items(Alignment::Center),
			row![
				column![
					horizontal_rule(10),
					files_disp	
	     		].spacing(20).max_width(400),
				column![
					horizontal_rule(10),
					annotation_disp,
					].spacing(20),
				].spacing(10),
			].spacing(10).padding(20);
			

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            //.center_x()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}