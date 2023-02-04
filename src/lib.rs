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
use std::io::ErrorKind;
use dirs::home_dir;



const NOTES_DIR: &str = ".filechest";

pub fn get_notes(file_path: &str) -> Option<String> {
	let test_path = home_dir().unwrap().display().to_string() + "/" + NOTES_DIR;
	let dir_create = fs::create_dir(test_path);
	match dir_create {
		Ok(_) => (),
		Err(error) => if error.kind() !=  ErrorKind::AlreadyExists {
			println!("{:?}", error);
		},
	};

	if let Ok(c_path) = fs::canonicalize(file_path){
		//println!("{:?}", c_path);
		return Some(format!("These are the notes for {}:\n", c_path.display()));
	}
	None
}