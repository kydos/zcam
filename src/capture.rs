//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use clap::{App, Arg};
use zenoh::net::*;
use zenoh::net::ResKey::*;
use opencv::{
    core,
    prelude::*,
    videoio,
};

#[async_std::main]
async fn main() {
    // initiate logging
    env_logger::init();

    let args = App::new("zenoh-net videocapture example")    
    .arg(Arg::from_usage("-m, --mode=[MODE] 'The zenoh session mode.")
        .possible_values(&["peer", "client"]).default_value("peer"))
    .arg(Arg::from_usage("-p, --path=[path] 'The zenoh path on which the video will be published."))
    .arg(Arg::from_usage("-e, --peer=[LOCATOR]...  'Peer locators used to initiate the zenoh session.'"))
        .get_matches();

    let config = Config::default()
        .mode(args.value_of("mode").map(|m| Config::parse_mode(m)).unwrap().unwrap())
        .add_peers(args.values_of("peer").map(|p| p.collect()).or_else(|| Some(vec![])).unwrap());

    let path = args.value_of("path").unwrap();

    println!("Openning session...");
    let session = open(config, None).await.unwrap();

    let reskey = RId(session.declare_resource(&path.into()).await.unwrap());
    let _publ = session.declare_publisher(&reskey).await.unwrap();

    #[cfg(feature = "opencv-32")]
    let mut cam = videoio::VideoCapture::new_default(0).unwrap();  // 0 is the default camera
    #[cfg(not(feature = "opencv-32"))]
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();  // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
    if !opened {
        panic!("Unable to open default camera!");
    }
    let mut encode_options = opencv::types::VectorOfi32::new();
    encode_options.push(opencv::imgcodecs::IMWRITE_JPEG_QUALITY);
    encode_options.push(90);

    loop {
        let mut frame = core::Mat::default().unwrap();
        cam.read(&mut frame).unwrap();
        // let mut reduced = Mat::default().unwrap();
        // opencv::imgproc::resize(&frame, &mut reduced, opencv::core::Size::new(150, 100), 0.0, 0.0 , opencv::imgproc::INTER_LINEAR).unwrap();
    
        let mut buf = opencv::types::VectorOfu8::new();
        opencv::imgcodecs::imencode(".jpeg", &frame, &mut buf, &encode_options).unwrap();

        // println!("{}x{} => {} bytes", frame.size().unwrap().width, frame.size().unwrap().height, buf.len());

        session.write(&reskey, buf.to_vec().into()).await.unwrap();
        async_std::task::sleep(std::time::Duration::new(0, 10000000)).await;
    }
}
