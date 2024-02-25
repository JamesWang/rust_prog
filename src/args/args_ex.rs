extern crate structopt;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String, //[1]

    #[structopt(short = "d", long = "dead")]
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn show_text_cat(options: &Options) {

    let eye = if options.dead { "x" } else { "o" };
    let message = &options.message;

    println!("{}", message.bright_blue().underline().on_purple());
    println!(" \\");
    println!(" \\");
    println!(" /\\_/\\");
    println!(" ( {eye} {eye} )", eye = eye.red().bold());
    println!(" =( I )=");
}

use failure::ResultExt;

fn show_picture(options: &Options) -> Result<(), failure::Error>{
    let cat_text = r#"
\\
  \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )="
"#;
    let eye = if options.dead { "x" } else { "o" };
    let mut cat_template = match  &options.catfile {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|_| format!("Could not read file {:?}", path))?,
        None => cat_text.to_string(),
    };
    let cat_picture = cat_template.replace("{eye}", eye);
    println!("{}", cat_picture);
    Ok(())
}
pub fn args_main() {
    let options = Options::from_args();
    //show_text_cat(&options);
    show_picture(&options).expect("TODO: panic message");
}

