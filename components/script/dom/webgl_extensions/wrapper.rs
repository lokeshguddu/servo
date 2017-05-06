/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use core::nonzero::NonZero;
use dom::bindings::js::MutNullableJS;
use dom::bindings::reflector::DomObject;
use dom::bindings::trace::JSTraceable;
use dom::webglrenderingcontext::WebGLRenderingContext;
use js::jsapi::JSObject;
use heapsize::HeapSizeOf;
use super::{WebGLExtension, WebGLExtensionManager};

pub trait WebGLExtensionWrapper: JSTraceable + HeapSizeOf {
    fn instance_or_init(&self,
                        ctx: &WebGLRenderingContext,
                        manager: &WebGLExtensionManager)
                        -> NonZero<*mut JSObject>;
    fn is_supported(&self, &WebGLExtensionManager) -> bool;
    fn enable(&self, manager: &WebGLExtensionManager);
    fn name(&self) -> &'static str;
}

#[must_root]
#[derive(JSTraceable, HeapSizeOf)]
pub struct TypedWebGLExtensionWrapper<T: WebGLExtension> {
    extension: MutNullableJS<T::Extension>
}

impl<T: WebGLExtension> TypedWebGLExtensionWrapper<T> {
    pub fn new() -> TypedWebGLExtensionWrapper<T> {
        TypedWebGLExtensionWrapper {
            extension: MutNullableJS::new(None)
        }
    }
}

impl<T: WebGLExtension + JSTraceable + HeapSizeOf> WebGLExtensionWrapper for TypedWebGLExtensionWrapper<T> {
    #[allow(unsafe_code)]
    fn instance_or_init(&self,
                        ctx: &WebGLRenderingContext,
                        manager: &WebGLExtensionManager)
                        -> NonZero<*mut JSObject> {
        let mut enabled = true;
        let extension = self.extension.or_init(|| {
            enabled = false;
            T::new(ctx)
        });
        if !enabled {
            self.enable(manager);
        }
        unsafe {
            NonZero::new(extension.reflector().get_jsobject().get())
        }
    }

    fn is_supported(&self, manager: &WebGLExtensionManager) -> bool {
        self.extension.get().is_some() || T::is_supported(manager)
    }

    fn enable(&self, manager: &WebGLExtensionManager) {
        T::enable(manager);
    }

    fn name(&self) -> &'static str {
        T::name()
    }
}
