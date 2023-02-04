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

use gtk::prelude::{BoxExt, ButtonExt, CheckButtonExt, GtkWindowExt};
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

struct AppModel {
    counter: u8,
}

enum AppMsg {
    Increment,
    Decrement,
	Toggle(bool),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            },
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            },
			AppMsg::Toggle(b) => {
				println!("Toggled! {b}");
			},
        }
        true
    }
}

struct AppWidgets {
    window: gtk::ApplicationWindow,
    vbox: gtk::Box,
    inc_button: gtk::Button,
    dec_button: gtk::Button,
    label: gtk::Label,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    /// Initialize the UI.
    fn init_view(model: &AppModel, _parent_widgets: &(), sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();
        vbox.set_margin_all(5);

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        // Connect the widgets
        window.set_child(Some(&vbox));
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

		let test = gtk::CheckButton::with_label("Test Label");
		vbox.append(&test);

        // Connect events
		let cb_sender = sender.clone();
        test.connect_toggled(move |t| {
            send!(cb_sender, AppMsg::Toggle(t.is_active()));
        });

        let btn_sender = sender.clone();
        inc_button.connect_clicked(move |_| {
            send!(btn_sender, AppMsg::Increment);
        });

        dec_button.connect_clicked(move |_| {
            send!(sender, AppMsg::Decrement);
        });


        Self {
            window,
            vbox,
            inc_button,
            dec_button,
            label,
        }
    }

    /// Return the root widget.
    fn root_widget(&self) -> Self::Root {
        self.window.clone()
    }

    /// Update the view to represent the updated model.
    fn view(&mut self, model: &AppModel, _sender: Sender<AppMsg>) {
        self.label.set_label(&format!("Counter: {}", model.counter));
    }
}

fn main() {
    let model = AppModel { counter: 0 };
    let app = RelmApp::new(model);
    app.run();
}