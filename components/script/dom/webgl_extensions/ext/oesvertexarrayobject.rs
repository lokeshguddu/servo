/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use canvas_traits::CanvasMsg;
use dom::bindings::codegen::Bindings::OESVertexArrayObjectBinding::{self, OESVertexArrayObjectMethods};
use dom::bindings::codegen::Bindings::OESVertexArrayObjectBinding::OESVertexArrayObjectConstants;
use dom::bindings::js::{MutNullableJS, Root};
use dom::bindings::reflector::{DomObject, Reflector, reflect_dom_object};
use dom::webglrenderingcontext::WebGLRenderingContext;
use dom::webglvertexarrayobjectoes::WebGLVertexArrayObjectOES;
use dom_struct::dom_struct;
use js::conversions::ToJSValConvertible;
use js::jsapi::JSContext;
use js::jsval::{JSVal, NullValue};
use ipc_channel::ipc::IpcSender;
use super::{constants as webgl, WebGLExtension, WebGLExtensionManager};
use webrender_traits::{self, WebGLCommand, WebGLError};

#[dom_struct]
pub struct OESVertexArrayObject {
    reflector_: Reflector,
    bound_vao: MutNullableJS<WebGLVertexArrayObjectOES>,
    #[ignore_heap_size_of = "Defined in ipc-channel"]
    renderer: IpcSender<CanvasMsg>
}

impl OESVertexArrayObject {
    fn new_inherited(renderer: IpcSender<CanvasMsg>) -> OESVertexArrayObject {
        Self {
            reflector_: Reflector::new(),
            bound_vao: MutNullableJS::new(None),
            renderer: renderer
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
        self.renderer.send(CanvasMsg::WebGL(WebGLCommand::CreateVertexArray(sender))).unwrap();

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
                    self.renderer.send(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(None))).unwrap();
                }
            }
            self.renderer.send(CanvasMsg::WebGL(WebGLCommand::DeleteVertexArray(vao.id()))).unwrap();
            vao.mark_deleted();
        }
    }

    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn IsVertexArrayOES(&self, vao: Option<&WebGLVertexArrayObjectOES>) -> bool {
        vao.map_or(false, |vao| !vao.is_deleted())
    }

    // https://www.khronos.org/registry/webgl/extensions/OES_vertex_array_object/
    fn BindVertexArrayOES(&self, vao: Option<&WebGLVertexArrayObjectOES>) -> () {
        if let Some(vao) = vao {
            if !vao.is_deleted() {
                self.renderer.send(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(Some(vao.id())))).unwrap();
                self.bound_vao.set(Some(&vao));
            }
        } else {
            self.renderer.send(CanvasMsg::WebGL(WebGLCommand::BindVertexArray(None))).unwrap();
            self.bound_vao.set(None);
        }
    }
}

impl WebGLExtension for OESVertexArrayObject {
    type Extension = OESVertexArrayObject;
    fn new(ctx: &WebGLRenderingContext) -> Root<OESVertexArrayObject> {
        reflect_dom_object(box OESVertexArrayObject::new_inherited(ctx.ipc_renderer()),
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
