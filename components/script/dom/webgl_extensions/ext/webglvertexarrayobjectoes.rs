/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::WebGLVertexArrayObjectOESBinding;
use dom::bindings::js::Root;
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use std::cell::Cell;
use webrender_traits::WebGLVertexArrayId;

#[dom_struct]
pub struct WebGLVertexArrayObjectOES {
    reflector_: Reflector,
    id: WebGLVertexArrayId,
    is_deleted: Cell<bool>,
}

impl WebGLVertexArrayObjectOES {
    fn new_inherited(id: WebGLVertexArrayId) -> WebGLVertexArrayObjectOES {
        Self {
            reflector_: Reflector::new(),
            id: id,
            is_deleted: Cell::new(false),
        }
    }

    pub fn new(global: &GlobalScope, id: WebGLVertexArrayId) -> Root<WebGLVertexArrayObjectOES> {
        reflect_dom_object(box WebGLVertexArrayObjectOES::new_inherited(id),
                           global,
                           WebGLVertexArrayObjectOESBinding::Wrap)
    }

    pub fn id(&self) -> WebGLVertexArrayId {
        self.id
    }

    pub fn is_deleted(&self) -> bool {
        self.is_deleted.get()
    }

    pub fn mark_deleted(&self) {
        self.is_deleted.set(true)
    }
}
