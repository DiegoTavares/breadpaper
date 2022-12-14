use std::fmt::format;
use std::fs;
use structopt::StructOpt;
use uuid::Uuid;

/// Command to interact with BreadPaper notes
///
/// bpp add -title "this is a title" --content "this is a content"
/// bpp add --edit
///
/// bpp rm --id 1
///
/// bpp search [--all] "is a"
#[derive(StructOpt, Debug)]
pub struct BppCli {
    #[structopt(subcommand)]
    subcommands: SubCommands,
}

#[derive(StructOpt, Debug)]
enum SubCommands {
    #[structopt(about = "Add new note")]
    Add(AddOpts),
    #[structopt(about = "Remove existing note")]
    Rm(RmOpts),
    #[structopt(about = "Search for note")]
    Search(SearchOpts),
}

#[derive(StructOpt, Debug)]
struct AddOpts {
    #[structopt(
    long,
    short = "t",
    long_help = "Note Title",
    required_unless_one(&["edit"]),
    conflicts_with_all(&["edit"]),
    )]
    title: Option<String>,

    #[structopt(
    long,
    short = "c",
    long_help = "Note Content",
    required_unless_one(&["edit"]),
    conflicts_with_all(&["edit"]),
    )]
    content: Option<String>,

    #[structopt(
    long,
    short = "e",
    long_help = "Edit using VIM",
    required_unless_one(&["title", "content"]),
    conflicts_with_all(&["title", "content"]),
    )]
    edit: bool,
}

#[derive(StructOpt, Debug)]
struct RmOpts {
    #[structopt(
    long,
    short = "i",
    long_help = "Id of note to be removed",
    )]
    id: String,
}


#[derive(StructOpt, Debug)]
struct SearchOpts {
    #[structopt(
    long,
    short = "a",
    long_help = "Search for both title and content",
    )]
    all: bool,

    #[structopt()]
    input: String
}

#[derive(Debug)]
pub enum BppCliError {

}

impl BppCli {
    pub fn run(&self) -> Result<i32, BppCliError> {
        match &self.subcommands {
            SubCommands::Add(add_opts) => {
                if add_opts.edit {
                    let tmp_file = format!("/tmp/note_{:}.bpp", Uuid::new_v4());
                    let cmd = format!("vim {tmp_file:}");
                    std::process::Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn()
                        .expect("Error: Failed to start VIM")  // TODO: Handle errors
                        .wait()
                        .expect("Error: Editor crashed"); // TODO: Handle errors
                    // Read tmp file
                    let user_text = fs::read_to_string(tmp_file)
                        .expect("Error: Failed to read tmp file"); // TODO: Handle errors

                    println!("Add with edit {:}", user_text)
                } else {
                    println!("Add with opts {:?}", add_opts)
                }
            }
            SubCommands::Rm(rm_opts) => {
                println!("Rm with opts {:?}", rm_opts)
            }
            SubCommands::Search(search_opts) => {
                println!("Search with opts {:?}", search_opts)
            }
        }

        Ok(0)
    }
}

fn main() {
    let cli = BppCli::from_args();
    match cli.run() {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
