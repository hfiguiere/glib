// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use translate::*;

use Source;
use SourceId;
use MainContext;

impl MainContext {

    // we need to manually wrap as gir doesn't deal with pointer.
    pub fn prepare(&self, priority: &mut i32) -> bool {
        unsafe {
            from_glib(ffi::g_main_context_prepare(self.to_glib_none().0, priority))
        }
    }

    pub fn find_source_by_id(&self, source_id: &SourceId) -> Option<Source> {
        unsafe {
            from_glib_none(ffi::g_main_context_find_source_by_id(self.to_glib_none().0,
                                                                 source_id.to_glib()))
        }
    }

}
