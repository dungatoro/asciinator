use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AsciinatorArgs {
    /// Path to the image
    #[arg(short, long)]
    pub image_path: String,

    #[arg(long = "scale")]
    pub scale_factor: u32,

    #[arg(long = "stretch", default_value_t=2)]
    pub stretch_factor: usize,

    #[arg(short, long, 
      default_value_t = String::from(
      r#"" .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#
      ))]
    pub ramp: String,
}

