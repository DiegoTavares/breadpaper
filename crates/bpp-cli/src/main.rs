use bpp_proto::bpp::api_client::ApiClient;
use bpp_proto::bpp::{AddRequest, RmRequest, SearchRequest};
use std::fmt::format;
use std::fs;
use structopt::StructOpt;
use tonic::transport::Channel;
use tonic::Request;
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
    #[structopt(long, short = "i", long_help = "Id of note to be removed")]
    id: String,
}

#[derive(StructOpt, Debug)]
struct SearchOpts {
    #[structopt(long, short = "a", long_help = "Search for both title and content")]
    all: bool,

    #[structopt()]
    query: String,
}

#[derive(Debug)]
pub enum BppCliError {
    FailedToAddNote,
    FailedToRmNote,
    InvalidParameters,
}

impl BppCli {
    pub async fn run(&self) -> Result<i32, BppCliError> {
        let mut client = ApiClient::connect("http://[::1]:8085")
            .await
            .expect("Failed to connect");

        match &self.subcommands {
            SubCommands::Add(add_opts) => {
                let mut title_out = Some("".to_string());
                let mut content_out = Some("".to_string());

                if add_opts.edit {
                    let tmp_file = format!("/tmp/note_{:}.bpp", Uuid::new_v4());
                    let cmd = format!("vim {tmp_file:}");
                    std::process::Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn()
                        .expect("Error: Failed to start VIM") // TODO: Handle errors
                        .wait()
                        .expect("Error: Editor crashed"); // TODO: Handle errors
                                                          // Read tmp file
                    let user_text =
                        fs::read_to_string(tmp_file).expect("Error: Failed to read tmp file"); // TODO: Handle errors

                    if let Some((title, content)) = user_text.split_once('\n') {
                        title_out = Some(title.to_string());
                        content_out = Some(content.to_string());
                    } else {
                        println!(
                            "Invalid text: More than one line required. (First line is the title)"
                        )
                    }
                } else {
                    title_out = add_opts.title.clone();
                    content_out = add_opts.content.clone();
                    println!("Add with opts {:?}", add_opts)
                }

                if title_out.is_some() && content_out.is_some() {
                    Self::handle_add(&mut client, &title_out.unwrap(), &content_out.unwrap()).await
                } else {
                    Err(BppCliError::InvalidParameters)
                }
            }
            SubCommands::Rm(rm_opts) => Self::handle_rm(&mut client, &rm_opts.id).await,
            SubCommands::Search(search_opts) => {
                Self::handle_search(&mut client, &search_opts.query, search_opts.all).await
            }
        }
    }

    async fn handle_add(
        client: &mut ApiClient<Channel>,
        title: &String,
        content: &String,
    ) -> Result<i32, BppCliError> {
        let request = Request::new(AddRequest {
            title: title.clone(),
            content: content.clone(),
        });

        let response = client.add(request).await;

        match response {
            Ok(resp) => {
                if let Some(note) = resp.into_inner().note {
                    println!("Note added! #{:}", note.id);
                    Ok(0)
                } else {
                    Ok(1)
                }
            }
            Err(err) => Err(BppCliError::FailedToAddNote),
        }
    }

    async fn handle_rm(client: &mut ApiClient<Channel>, id: &String) -> Result<i32, BppCliError> {
        let request = Request::new(RmRequest { id: id.clone() });

        let response = client.rm(request).await;

        match response {
            Ok(resp) => {
                if let Some(note) = resp.into_inner().note {
                    println!("Note removed! #{:}", note.id);
                    Ok(0)
                } else {
                    Ok(1)
                }
            }
            Err(err) => Err(BppCliError::FailedToRmNote),
        }
    }

    async fn handle_search(
        client: &mut ApiClient<Channel>,
        query: &String,
        all: bool,
    ) -> Result<i32, BppCliError> {
        let request = Request::new(SearchRequest {
            query: query.clone(),
            all,
        });

        let response = client.search(request).await;

        match response {
            Ok(resp) => {
                let notes = resp.into_inner().notes;
                if notes.len() > 0 {
                    for note in notes {
                        println!("{}: {}", note.title, note.content);
                        println!("--------");
                    }
                } else {
                    println!("Found not notes!");
                }
                Ok(0)
            }
            Err(err) => Err(BppCliError::FailedToRmNote),
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = BppCli::from_args();
    match cli.run().await {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
