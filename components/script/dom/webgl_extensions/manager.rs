/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use core::iter::FromIterator;
use core::nonzero::NonZero;
use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::WebGLRenderingContextBinding::WebGLRenderingContextConstants as constants;
use dom::bindings::trace::JSTraceable;
use dom::webglrenderingcontext::WebGLRenderingContext;
use js::jsapi::JSObject;
use heapsize::HeapSizeOf;
use super::ext;
use super::WebGLExtension;
use super::wrapper::{WebGLExtensionWrapper, TypedWebGLExtensionWrapper};
use std::collections::{HashMap, HashSet};

type GLenum = u32;

const default_disabled_tex_types: [GLenum; 2] = [
    constants::FLOAT, constants::HALF_FLOAT
];

#[must_root]
#[derive(JSTraceable, HeapSizeOf)]
pub struct WebGLExtensionManager {
    extensions: DOMRefCell<HashMap<&'static str, Box<WebGLExtensionWrapper>>>,
    gl_extensions: DOMRefCell<HashSet<String>>,
    disabled_tex_types: DOMRefCell<HashSet<GLenum>>,
    effective_tex_internal_formats: DOMRefCell<HashMap<TexFormatType,u32>>
}

impl WebGLExtensionManager {
    pub fn new() -> WebGLExtensionManager {
        Self {
            extensions: DOMRefCell::new(HashMap::new()),
            gl_extensions: DOMRefCell::new(HashSet::new()),
            disabled_tex_types: DOMRefCell::new(default_disabled_tex_types.iter().cloned().collect()),
            effective_tex_internal_formats: DOMRefCell::new(HashMap::new()),
        }
    }

    pub fn init_once<F>(&self, cb: F) where F: FnOnce() -> String {
        if self.extensions.borrow().len() == 0 {
            let gl_str = cb();
            *self.gl_extensions.borrow_mut() = HashSet::from_iter(gl_str.split(&[',',' '][..]).map(|s| s.into()));
            self.register_all_extensions();
        }
    }

    pub fn register<T:'static + WebGLExtension + JSTraceable + HeapSizeOf>(&self) {
        self.extensions.borrow_mut().insert(T::name(), box TypedWebGLExtensionWrapper::<T>::new());
    }

    pub fn get_suported_extensions(&self) -> Vec<&'static str> {
        self.extensions.borrow().iter()
                                .filter(|ref v| v.1.is_supported(&self))
                                .map(|ref v| v.1.name())
                                .collect()
    }

    pub fn get_or_init_extension(&self, name: &str, ctx: &WebGLRenderingContext) -> Option<NonZero<*mut JSObject>> {
        self.extensions.borrow().get(name).and_then(|extension| {
            if extension.is_supported(self) {
                Some(extension.instance_or_init(ctx, self))
            } else {
                None
            }
        })
    }

    pub fn supports_gl_extension(&self, name: &str) -> bool {
        self.gl_extensions.borrow().contains(name)
    }

    pub fn supports_any_gl_extension(&self, names: &[&str]) -> bool {
        let gl_ext = self.gl_extensions.borrow();
        names.iter().any(|name| gl_ext.contains(*name))
    }

    pub fn enable_tex_type(&self, data_type: GLenum) {
        self.disabled_tex_types.borrow_mut().remove(&data_type);
    }

    pub fn is_tex_type_enabled(&self, data_type: GLenum) -> bool {
        self.disabled_tex_types.borrow().get(&data_type).is_none()
    }

    pub fn add_effective_tex_internal_format(&self,
                                             source_internal_format: u32,
                                             source_data_type: u32,
                                             effective_internal_format: u32)
    {
        let format = TexFormatType(source_internal_format, source_data_type);
        self.effective_tex_internal_formats.borrow_mut().insert(format,
                                                                effective_internal_format);

    }

    pub fn get_effective_tex_internal_format(&self,
                                             source_internal_format: u32,
                                             source_data_type: u32) -> u32 {
        let format = TexFormatType(source_internal_format, source_data_type);
        *(self.effective_tex_internal_formats.borrow().get(&format)
                                                      .unwrap_or(&source_internal_format))
        
    }

    fn register_all_extensions(&self) {
        self.register::<ext::oestexturefloat::OESTextureFloat>();
    }
}

// Helper struct
#[derive(JSTraceable, HeapSizeOf, PartialEq, Eq, Hash)]
struct TexFormatType(u32, u32);