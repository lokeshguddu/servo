/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// https://w3c.github.io/webvr/#interface-navigator
interface VR: EventTarget {
  Promise<sequence<VRDisplay>> getVRDisplays();
  //readonly attribute FrozenArray<VRDisplay> activeVRDisplays;
  readonly attribute boolean vrEnabled;
};