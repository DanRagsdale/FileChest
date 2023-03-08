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

#[derive(Debug)]
pub enum FileElementInput {}

#[derive(Debug)]
pub enum FileElementOutput {}

#[derive(Debug)]
pub enum AppMsg {
    SetDir(String),
    SetDirFromSelected,
	SetShowHidden(bool),
	SelectFile(i32),
	SubmitNote,
	SubmitTags(String),
	ShowFileContext(f64, f64),
	OpenCurrentFile(OpenType),
}

#[derive(Debug)]
pub enum OpenType {
	OpenFile,
	OpenParent,
}