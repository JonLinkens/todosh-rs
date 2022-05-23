use clap::{Parser, Subcommand};

mod cmd;

#[derive(Debug, Parser)]
#[clap(name = "todosh")]
#[clap(author="Jon Linkens", version="0.1.0", about = "a lightweight CLI todo list manager", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// create a new todo list
    Create {},

    /// lists tasks to complete
    List {},

    /// appends a task to the list
    #[clap(arg_required_else_help = true)]
    Add {
        /// task description
        task: String,
    },

    /// finish a task
    #[clap(arg_required_else_help = true)]
    Finish {
        /// number of task to finish
        tasknum: i32,
    },

    /// delete a task list
    Delete {},
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Create {} => {
            cmd::create_list();
        }
        Commands::List {} => {
            cmd::list_tasks();
        }
        Commands::Add { task } => {
            cmd::add_task(task);
        }
        Commands::Finish { tasknum } => {
            cmd::finish_task(tasknum);
        }
        Commands::Delete {} => {
            cmd::delete_list();
        }
    }
}
