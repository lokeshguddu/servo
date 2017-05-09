/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::CanvasMsg;
use dom::bindings::codegen::Bindings::OESVertexArrayObjectBinding::{self, OESVertexArrayObjectMethods};
use dom::bindings::codegen::Bindings::OESVertexArrayObjectBinding::OESVertexArrayObjectConstants;
use dom::bindings::js::{JS, MutNullableJS, Root};
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::webglrenderingcontext::WebGLRenderingContext;
use dom::webglvertexarrayobjectoes::WebGLVertexArrayObjectOES;
use dom_struct::dom_struct;
use js::conversions::ToJSValConvertible;
use js::jsapi::JSContext;
use js::jsval::{JSVal, NullValue};
use super::{WebGLExtension, WebGLExtensionManager};
use webrender_traits::{self, WebGLCommand, WebGLError};

#[dom_struct]
pub struct OESVertexArrayObject {
    reflector_: Reflector,
    ctx: JS<WebGLRenderingContext>,
    bound_vao: MutNullableJS<WebGLVertexArrayObjectOES>,
}

impl OESVertexArrayObject {
    fn new_inherited(ctx: &WebGLRenderingContext) -> OESVertexArrayObject {
        Self {
            reflector_: Reflector::new(),
            ctx: JS::from_ref(ctx),
            bound_vao: MutNullableJS::new(None)
        }
    }

    #[allow(unsafe_code)]
    fn get_current_binding(&self, cx:*mut JSContext) -> JSVal {
        rooted!(in(cx) let mut rval = NullValue());
        if let Some(bound_vao) = self.bound_vao.get() {
            unsafe {
                bound_vao.to_jsval(cx, rval.handle_mut());
            }
        }
        rval.get()
    }
}

impl OESVertexArrayObjectMethods for OESVertexArrayObject {
    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn CreateVertexArrayOES(&self) -> Option<Root<WebGLVertexArrayObjectOES>> {
        let (sender, receiver) = webrender_traits::channel::msg_channel().unwrap();
        self.ctx.send_renderer_message(CanvasMsg::WebGL(WebGLCommand::CreateVertexArray(sender)));

        let result = receiver.recv().unwrap();
        result.map(|vao_id| WebGLVertexArrayObjectOES::new(&self.global(), vao_id))
    }

    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn DeleteVertexArrayOES(&self, vao: Option<&WebGLVertexArrayObjectOES>) -> () {
        if let Some(vao) = vao {
            // Unbind deleted VAO if currently bound
            if let Some(bound_vao) = self.bound_vao.get() {
                if bound_vao.id() == vao.id() {
                    self.bound_vao.set(None);
                    self.ctx.send_renderer_message(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(None)));
                }
            }
            self.ctx.send_renderer_message(CanvasMsg::WebGL(WebGLCommand::DeleteVertexArray(vao.id())));
            vao.set_deleted();
        }
    }

    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn IsVertexArrayOES(&self, vao: Option<&WebGLVertexArrayObjectOES>) -> bool {
        // Conformance tests expect false if vao never bound
        vao.map_or(false, |vao| !vao.is_deleted() && vao.ever_bound())
    }

    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn BindVertexArrayOES(&self, vao: Option<&WebGLVertexArrayObjectOES>) -> () {
        if let Some(bound_vao) = self.bound_vao.get() {
            // Store current attrib array bindings
            let buffer_array = self.ctx.bound_buffer_array();
            let buffer_array_element = self.ctx.bound_buffer_element_array();
            bound_vao.set_bound_buffer_array(buffer_array.as_ref().map(|b| &**b));
            bound_vao.set_bound_buffer_element_array(buffer_array_element.as_ref().map(|b| &**b));
        }

        if let Some(vao) = vao {
            if vao.is_deleted() {
                self.ctx.webgl_error(WebGLError::InvalidOperation);
                return;
            }

            self.ctx.send_renderer_message(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(Some(vao.id()))));
            vao.set_ever_bound();
            self.bound_vao.set(Some(&vao));

            // Restore WebGLRenderingContext current bindings
            let buffer_array = vao.bound_buffer_array();
            let buffer_array_element = vao.bound_buffer_element_array();
            self.ctx.set_bound_buffer_array(buffer_array.as_ref().map(|b| &**b));
            self.ctx.set_bound_buffer_element_array(buffer_array_element.as_ref().map(|b| &**b));
        } else {
            self.ctx.send_renderer_message(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(None)));
            self.bound_vao.set(None);
            self.ctx.set_bound_buffer_array(None);
            self.ctx.set_bound_buffer_element_array(None);
        }
    }
}

impl WebGLExtension for OESVertexArrayObject {
    type Extension = OESVertexArrayObject;
    fn new(ctx: &WebGLRenderingContext) -> Root<OESVertexArrayObject> {
        reflect_dom_object(box OESVertexArrayObject::new_inherited(ctx),
                           &*ctx.global(),
                           OESVertexArrayObjectBinding::Wrap)
    }

    fn is_supported(manager: &WebGLExtensionManager) -> bool {
        manager.supports_any_gl_extension(&["GL_OES_vertex_array_object",
                                            "GL_ARB_vertex_array_object",
                                            "GL_APPLE_vertex_array_object"])
    }

    fn enable(manager: &WebGLExtensionManager) {
        let query = OESVertexArrayObjectConstants::VERTEX_ARRAY_BINDING_OES;
        manager.add_query_parameter_handler(query, Box::new(|cx, webgl_ctx| {
            match webgl_ctx.get_extension_manger().get_dom_object::<OESVertexArrayObject>() {
                Some(dom_object) => {
                    Ok(dom_object.get_current_binding(cx))
                },
                None => {
                    // Extension instance not found!
                    Err(WebGLError::InvalidOperation)
                }
            }
        }));
    }

    fn name() -> &'static str {
        "OES_vertex_array_object"
    }
}
