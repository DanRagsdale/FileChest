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

//use std::fs;
//use std::io::ErrorKind;
use std::fs::DirEntry;
use std::path::PathBuf;

use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::DirEntryExt;

//use dirs::home_dir;

use rusqlite::{Connection, Result};


const NOTES_DIR: &str = ".filechest";

#[derive(Debug, Default, Clone)]
pub struct FileRef {
	pub file_path: PathBuf,
	pub inode: u64,
}

impl FileRef {
	pub fn from_pathbuf(pb: &PathBuf) -> Result<Self, std::io::Error> {
		let file_path = pb.canonicalize()?;
		println!("Raw {:?} , Canonical: {:?}", pb, file_path);
		let m = std::fs::symlink_metadata(pb)?;
		let inode = m.ino();
		
		Ok(Self { file_path: pb.clone(), inode, })
	}

	pub fn from_direntry(de: &DirEntry) -> Result<Self, std::io::Error> {
		Ok(Self { file_path: de.path(), inode: de.ino()})
	}
}

pub struct NotesDB {
	conn: Connection,
}

impl NotesDB {
	pub fn build() -> Result<Self> {
		let conn = Connection::open("test.db")?;
		conn.execute(
			"create table if not exists file_notes (
			     inode integer primary key,
			     note text
			)",
			()
		)?;

		Ok(NotesDB {
			conn
		})
	}

	pub fn get_note(&self, file_ref: &FileRef) -> Result<String, rusqlite::Error> {
		//let test_path = home_dir().unwrap().display().to_string() + "/" + NOTES_DIR;
		//let dir_create = fs::create_dir(test_path);
		//match dir_create {
		//	Ok(_) => (),
		//	Err(error) => if error.kind() !=  ErrorKind::AlreadyExists {
		//		println!("{:?}", error);
		//	},
		//};

		//if let Ok(c_path) = fs::canonicalize(file_path){
		//	let inode = file.ino();

		//	//println!("{:?}", c_path);
		//	return Some(format!("These are the notes for {}:\n", c_path.display()));
		//}
		self.conn.query_row("SELECT note FROM file_notes WHERE inode=:inode", &[(":inode", &file_ref.inode.to_string())], |row| {
			row.get::<usize, String>(0)
		})
	}

	pub fn set_note(&self, file_ref: &FileRef, note: &str) -> Result<(), rusqlite::Error> {
		self.conn.execute(
        	"INSERT OR REPLACE INTO file_notes (inode, note) VALUES (?1, ?2)",
        	(&file_ref.inode, note),)?;
		Ok(())
	}
}