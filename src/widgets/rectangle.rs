// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use ffi::PangoRectangle;
use libc::c_int;
//use std::default::Default;

/// The PangoRectangle structure represents a rectangle. It is frequently used to represent the
/// logical or ink extents of a single glyph or section of text. (See, for instance,
/// pango_font_get_glyph_extents()).
pub trait Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self;
}

impl Rectangle for PangoRectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> PangoRectangle {
        PangoRectangle {
            x: x as c_int,
            y: y as c_int,
            width: width as c_int,
            height: height as c_int,
        }
    }
}

/*impl Default for Rectangle {
    fn default() -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0
        }
    }
}*/