// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;

pub struct SourceFuncs(ffi::GSourceFuncs);


impl<'a> ToGlibPtrMut<'a, *mut ffi::GSourceFuncs>  for SourceFuncs {
    type Storage = &'a mut SourceFuncs;

    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GSourceFuncs, Self> {
        StashMut(&mut self.0, self)
    }
}
