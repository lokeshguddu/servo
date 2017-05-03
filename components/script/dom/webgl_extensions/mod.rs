/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod ext;
mod extension;
mod manager;
mod wrapper;

// Some extra constants not exposed in WebGLRenderingContext constants
pub mod ext_constants {
   pub const RGBA32F: u32 = 0x8814;
   pub const RGB32F: u32 = 0x8815;
   pub const RGBA16F: u32 = 0x881A;
   pub const RGB16F: u32 = 0x881B;
}

pub use self::extension::WebGLExtension;
pub use self::manager::WebGLExtensionManager;