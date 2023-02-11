use file_chest::FileRef;
use crate::messages::*;

use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct Task {
	pub file: FileRef,
    pub completed: bool,
}

#[relm4::factory(pub)]
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