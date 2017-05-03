/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::OESTextureFloatBinding;
use dom::bindings::codegen::Bindings::WebGLRenderingContextBinding::WebGLRenderingContextConstants as webgl;
use dom::bindings::js::Root;
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::globalscope::GlobalScope;
use dom::webglrenderingcontext::WebGLRenderingContext;
use dom_struct::dom_struct;
use super::{ext_constants as gl, WebGLExtension, WebGLExtensionManager};

#[dom_struct]
pub struct OESTextureFloat {
    reflector_: Reflector,
}

impl OESTextureFloat {
    pub fn new_inherited() -> OESTextureFloat {
        Self {
            reflector_: Reflector::new(),
        }
    }
}

impl WebGLExtension for OESTextureFloat {
    type Extension = OESTextureFloat;
    fn new(global: &GlobalScope, ctx: &WebGLRenderingContext) -> Root<OESTextureFloat> {
        reflect_dom_object(box OESTextureFloat::new_inherited(),
                           global,
                           OESTextureFloatBinding::Wrap)
    }

    fn is_supported(manager: &WebGLExtensionManager) -> bool {
        manager.supports_any_gl_extension(&["GL_OES_texture_float",
                                            "GL_ARB_texture_float"])
    }

    fn enable(manager: &WebGLExtensionManager) {
        // Enable FLOAT text data type
        manager.enable_tex_type(webgl::FLOAT);
        let needs_replace = !manager.supports_gl_extension("GL_OES_texture_float");
        if needs_replace {
            // Special internal formast must be used to avoid clamped float values
            manager.add_effective_tex_internal_format(webgl::RGBA, webgl::FLOAT, gl::RGBA32F);
        }
    }

    fn name() -> &'static str {
        "OES_texture_float"
    }
}