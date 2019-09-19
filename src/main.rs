extern crate tail;
extern crate inotify;

use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

use tail::BackwardsReader;
use std::io::{BufReader, BufWriter};
use std::fs::File;

const DEFAULT_DIR: &str = ".";


fn main() {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    let current_dir = std::env::args().nth(1).unwrap_or(DEFAULT_DIR.to_string());
    let mut split = current_dir.split("/");

    let vec: Vec<&str> = split.collect();
    println!("{:?}", vec[vec.len()-1]);
    if (vec.len() <= 0) {
        println!("local_file");
    }

    inotify.add_watch(
            current_dir,
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        );

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) {
                if event.mask.contains(EventMask::ISDIR) {
                    // println!("Directory created: {:?}", event.name);
                } else {
                    print_log_msg();
                    // println!("File created: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::DELETE) {
                if event.mask.contains(EventMask::ISDIR) {
                    // println!("Directory deleted: {:?}", event.name);
                } else {
                    //println!("File deleted: {:?}", event.name);
                }
            } else if event.mask.contains(EventMask::MODIFY) {
                if event.mask.contains(EventMask::ISDIR) {
                    // println!("Directory modified: {:?}", event.name);
                } else {
                    print_log_msg();
                }
            }
        }
    }
}


fn print_log_msg() {
    let filename = std::env::args().nth(1).unwrap_or(DEFAULT_DIR.to_string());
    let fd = File::open(filename).unwrap();
    let mut fd = BufReader::new(fd);
    let mut reader = BackwardsReader::new(1, &mut fd);

    let mut out = BufWriter::new(std::io::stdout());
    reader.read_all(&mut out);
}
