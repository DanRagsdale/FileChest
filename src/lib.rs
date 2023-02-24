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
		//let file_path = pb.canonicalize()?;
		//println!("Raw {:?} , Canonical: {:?}", pb, file_path);
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
			"CREATE TABLE IF NOT EXISTS file_notes (
				inode INTEGER PRIMARY KEY,
				known_path TEXT,
				note TEXT
			);",
			()
		)?;
		
		conn.execute(
			"CREATE TABLE IF NOT EXISTS file_tags (
				id INTEGER PRIMARY KEY AUTOINCREMENT,
				tag_name VARCHAR(255) UNIQUE
			);",
			()
		)?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS tag_relations (
				--relation_id INTEGER PRIMARY KEY AUTOINCREMENT,
				tag_id INTEGER NOT NULL,
				file_id INTEGER NOT NULL,

				FOREIGN KEY(tag_id) REFERENCES file_tags(id),	
				FOREIGN KEY(file_id) REFERENCES file_notes(inode),

				CONSTRAINT uc_tfid UNIQUE (tag_id, file_id)
			);",
			()
		)?;

		Ok(NotesDB {
			conn
		})
	}

	pub fn get_note(&self, file_ref: &FileRef) -> Result<String, rusqlite::Error> {
		self.conn.execute(
			"UPDATE file_notes SET known_path = ?1 WHERE inode = ?2;",
			(file_ref.file_path.to_str().unwrap(), &file_ref.inode)
		)?;

		self.get_note_no_update(file_ref)
	}
	
	pub fn get_note_no_update(&self, file_ref: &FileRef) -> Result<String, rusqlite::Error> {
		self.conn.query_row(
			"SELECT note FROM file_notes WHERE inode=?1;",
			(file_ref.inode,),
			|row| { row.get::<usize, String>(0)}
		)
	}

	pub fn set_note(&self, file_ref: &FileRef, note: &str) -> Result<(), rusqlite::Error> {
		self.conn.execute(
        	"INSERT OR REPLACE INTO file_notes(inode, known_path, note) VALUES(?1, ?2, ?3);",
        	(file_ref.inode, file_ref.file_path.to_str().unwrap(), note, ),
		)?;
		Ok(())
	}

	pub fn add_file(&self, file_ref: &FileRef) -> Result<(), rusqlite::Error> {
		self.conn.execute(
			"INSERT OR IGNORE INTO file_notes(inode, known_path) VALUES(?1, ?2);",
			(file_ref.inode, file_ref.file_path.to_str().unwrap(), )
		)?;
		Ok(())
	}

	pub fn add_tag(&self, file_ref: &FileRef, tag: &str) -> Result<(), rusqlite::Error> {
		//Check if we have a corresponding tag in the tags table. Add the new tag if we don't.
		self.conn.execute(
			"INSERT OR IGNORE INTO file_tags(tag_name) VALUES(?1);",
			(tag,),
		)?;
		
		let tag_id = self.conn.query_row(
			"SELECT id FROM file_tags WHERE tag_name=?1",
			(tag,),
			|row| { row.get::<usize, usize>(0)}
		)?;

		self.add_file(file_ref)?;

		self.conn.execute(
			"INSERT OR IGNORE INTO tag_relations(tag_id, file_id) VALUES(?1, ?2);",
			(tag_id, file_ref.inode),
		)?;
		println!("{tag_id}");
		//Add the tag_reference
		Ok(())
	}
}