mod args;
mod validation;

use clap::Parser;

use crate::args::Arguments;
use crate::validation::check_args;
use std::io;
use std::process::Command;
use std::thread::available_parallelism;
use std::time::Instant;
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
pub struct VideoError {
    reason: String,
}
impl From<io::Error> for VideoError {
    fn from(e: io::Error) -> VideoError {
        VideoError {
            reason: e.to_string(),
        }
    }
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
    let a = Arguments::parse();
    let res = check_args(&a);
    println!("{:?}", res);
}
