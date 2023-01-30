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

use iced::theme::Theme;
use iced::widget::{text, button, column, container, horizontal_rule, row, scrollable, text_input, vertical_space,};
use iced::{Element, Length, Sandbox, Settings};

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
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DirSubmit,
}

impl Sandbox for ChestApp {
    type Message = Message;

    fn new() -> Self {
        ChestApp {theme: Theme::Light, input_value: String::from(""), search_dir: String::from("./")}
    }

    fn title(&self) -> String {
        String::from("File Chest")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::DirSubmit => self.search_dir = self.input_value.clone(),
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

		let mut files = column![];

		let paths = fs::read_dir(&self.search_dir).unwrap();
		for path in paths {
			files = files.push(text(format!("{}", path.unwrap().path().display())));
		};

        let scrollable = scrollable(
            column![
                "Scroll me!\nLineTest\nLine3Test\nAnotherOne",
                vertical_space(Length::Units(1000)),
                "Test"
            ]
            .width(Length::Fill),
        );

        let content = column![
            row![text_input, button].spacing(10),
			row![
				column![
					horizontal_rule(10),
					files
	     		].spacing(20).max_width(300),
				column![
					horizontal_rule(10),
					scrollable,
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