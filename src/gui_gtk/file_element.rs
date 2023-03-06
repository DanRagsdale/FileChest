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

use file_chest::FileRef;
use crate::messages::*;

use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct FileElement {
	pub file: FileRef,
    pub completed: bool,
}

#[relm4::factory(pub)]
impl FactoryComponent for FileElement {
	type Init = FileRef;
	type Input = FileElementInput;
	type Output = FileElementOutput;
	type CommandOutput = ();
	type ParentInput = AppMsg;
	type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            
			#[name(label)]
            gtk::Label {
                set_label: &self.file.file_path.file_name().unwrap().to_str().unwrap(),
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                set_margin_all: 12,
				//set_selectable: true,
            },
        }
    }

    fn pre_view() {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn output_to_parent_input(_output: Self::Output) -> Option<AppMsg> {
		None
    }

    fn init_model(file: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
			file,
            completed: false,
        }
    }
}