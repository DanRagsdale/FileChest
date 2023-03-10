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

mod gui_model;
mod messages;
mod file_element;

use file_chest::NotesDB;

use crate::gui_model::AppModel;
use relm4::prelude::*;

fn main() {
    let app = RelmApp::new("com.danielragsdale.file_chest");
	let db = NotesDB::build().expect("Could not load database");

    app.run::<AppModel>(db);
}