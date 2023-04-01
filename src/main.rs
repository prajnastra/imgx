use clap::Parser;
use image::image_dimensions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let (w, h) = image_dimensions(args.path).expect("Not able to read");

    println!("Dimensions {}x{}", w, h);
}
