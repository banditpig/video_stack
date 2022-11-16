mod args;
mod validation;

use clap::{arg, Parser};
use std::fmt::{Display, Formatter};

use crate::args::Arguments;
use crate::validation::check_args;
use std::ffi::OsStr;
use std::fs::{DirEntry, ReadDir};
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
use std::thread::available_parallelism;
use std::time::Instant;
use std::{fs, io};
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
pub struct VideoError {
    pub reason: String,
}

impl Display for VideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}
impl From<io::Error> for VideoError {
    fn from(e: io::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
}

fn files_in_folder(folder: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];

    for path in fs::read_dir(folder)? {
        let path = path?.path();
        files.push(path.to_owned());
    }
    Ok(files)
}
fn process_videos(args: &Arguments) -> Result<(), VideoError> {
    let cores = available_parallelism().unwrap().get();
    let pool = ThreadPool::with_name("worker".into(), 8);

    let client_vids = files_in_folder(&args.client_folder)?; //.unwrap();
    let dummy_vids = files_in_folder(&args.dummies_folder)?;
    let mut total = 0;
    let now = Instant::now();
    for cvid in client_vids {
        for ix in 0..args.quantity {
            let vid1 = format!("{}", cvid.display());
            let vid2 = format!("{}", dummy_vids[ix].display());
            let outname = format!("stacked_video{}.mp4", total);
            total += 1;
            pool.execute(move || {

                //let out_name = format!("output{}.mp4", ix);
                println!("Working on  {outname}");
                let output = Command::new("ffmpeg")
                    .args([
                        "-hide_banner",
                        "-loglevel", "error",
                        "-y",
                        "-threads", "2",
                        "-i", &vid1,
                        "-i", &vid2,
                        "-filter_complex", "[0:v]scale=1080:1920,crop=in_w:in_h/2:in _w:in_h/4[v0];[1:v]scale=1080:1920,crop=in_w:in_h/2:in_w:in_h/4[v1];[v0][v1]vstack",
                        "-c:v", "libx264",
                        &outname
                    ])
                    .status();


                match output {
                    Ok(status) => if status.success() {
                        println!("Stacked video {outname} created." );
                    } else {
                        println!("Failed to create {}", outname);
                    }

                    Err(e) => {println!("Finished {} with error {:?}", outname, e);}
                }

            });
        }
    }
    pool.join();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

fn in_parallel(cnt: usize) {
    let cores = available_parallelism().unwrap().get();
    println!("{cores}");
    let now = Instant::now();

    let pool = ThreadPool::with_name("worker".into(), 8);
    for i in 0..cnt {
        pool.execute(move || {
            let out_name = format!("output{}.mp4", i);
            println!("Working on  {out_name}");
            let output = Command::new("ffmpeg")
                .args([
                    "-hide_banner",
                    "-loglevel", "error",
                    "-y",
                    "-threads", "2",
                    "-i", "vid1.mp4",
                    "-i", "vid2.mp4",
                    "-filter_complex", "[0:v]scale=1080:1920,crop=in_w:in_h/2:in _w:in_h/4[v0];[1:v]scale=1080:1920,crop=in_w:in_h/2:in_w:in_h/4[v1];[v0][v1]vstack",
                    "-c:v", "libx264",
                    &out_name
                ])
                .status();


            match output {
                Ok(status) => if status.success() {
                    println!("Stacked video {out_name} created." );
                } else {
                    println!("Failed to create {}", out_name);
                }

                Err(e) => {println!("Finished {} with error {:?}", out_name, e);}
            }

        });
    }
    pool.join();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
fn main() {
    let args = Arguments::parse();
    let res = check_args(&args);
    match res {
        Ok(_) => process_videos(&args).unwrap(),
        Err(e) => {
            println!("{}", e);
        }
    }
}
