/*
 * Copyright 2025 Security Union LLC
 *
 * Licensed under either of
 *
 * * Apache License, Version 2.0
 *   (http://www.apache.org/licenses/LICENSE-2.0)
 * * MIT license
 *   (http://opensource.org/licenses/MIT)
 *
 * at your option.
 *
 * Unless you explicitly state otherwise, any contribution intentionally
 * submitted for inclusion in the work by you, as defined in the Apache-2.0
 * license, shall be dual licensed as above, without any additional terms or
 * conditions.
 */

//! This crate provides a client-side (browser) interface to the videocall protocol.  The purpose is to
//! take care of the media I/O both for the encoding the current participant and for rendering the
//! media from the remote peers.  It also provides tools for listing available media devices and
//! granting access.
//!
//! This crate intends to make no assumptions about the UI or the HTML of the client app.
//! The only DOM data it needs is the ID of the `HtmlVideoElement` for the participant's own video
//! display and the ID's of the `HtmlCanvasElement`s into which remote peer video should be renderered.
//!
//! In addition to its use by Rust UI apps (e.g. via yew), it is intended that this crate be
//! compiled to npm module that could be called from javascript, e.g. in an electron app.
//!
//! Currently, only the Chrome browser is supported, due to some of the Web APIs that are used.
//!
//! **NOTE:** This initial version is a slightly frankenstein result of piecemeal refactoring bits
//! from the original app and stitching them together.   It could use cleaning up both the API the
//! internal design.
//!
//! # Outline of usage
//!
//! For more detailed documentation see the doc for each struct.
//!
//! ## Client creation and connection:
//! ```no_run
//! let options = VideoCallClientOptions {...}; // set parameters and callbacks for various events
//! let client = VideoCallClient::new(options);
//!
//! client.connect();
//! ```
//!
//! ## Encoder creation:
//! ```no_run
//! let camera = CameraEncoder.new(client, video_element_id);
//! let microphone = MicrophoneEncoder.new(client);
//! let screen = ScreenEncoder.new(client);
//!
//! camera.select(video_device);
//! camera.start();
//! camera.stop();
//! microphone.select(video_device);
//! microphone.start();
//! microphone.stop();
//! screen.start();
//! screen.stop();
//! ```
//!
//! ## Device access permission:
//!
//! ```no_run
//! let media_device_access = MediaDeviceAccess::new();
//! media_device_access.on_granted = ...; // callback
//! media_device_access.on_denied = ...; // callback
//! media_device_access.request();
//! ```
//!
//! ### Device query and listing:
//! ```no_run
//! let media_device_list = MediaDeviceList::new();
//! media_device_list.audio_inputs.on_selected = ...; // callback
//! media_device_access.video_inputs.on_selected = ...; // callback
//!
//! media_device_list.load();
//!
//! let microphones = media_device_list.audio_inputs.devices();
//! let cameras = media_device_list.video_inputs.devices();
//! media_device_list.audio_inputs.select(&microphones[i].device_id);
//! media_device_list.video_inputs.select(&cameras[i].device_id);
//!
//! ```

pub mod audio_worklet_codec;
mod client;
mod connection;
pub mod constants;
pub mod crypto;
pub mod decode;
pub mod diagnostics;
pub mod encode;
mod media_devices;
pub mod utils;
mod wrappers;
pub use client::{VideoCallClient, VideoCallClientOptions};
pub use decode::{
    create_audio_peer_decoder, AudioPeerDecoderTrait, PeerDecodeManager, VideoPeerDecoder,
};
pub use encode::{create_microphone_encoder, CameraEncoder, MicrophoneEncoderTrait, ScreenEncoder};
pub use media_devices::{MediaDeviceAccess, MediaDeviceList, SelectableDevices};
