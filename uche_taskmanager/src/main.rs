use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::error::Error;
use std::path::Path;

#[derive(Parser)]
struct UserArgs {
    #[clap(short, long, value_parser)]
    add: Option<String>,

    #[clap(short, long, value_parser)]
    done: Option<usize>,

    #[clap(short, long, value_parser)]
    list: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let u_arg = UserArgs::parse();
    let database_name = "cli_task.db";

    if let Some(task_to_add) = u_arg.add {
        add_task(database_name, &task_to_add)?; 
    } else if let Some(done_id) = u_arg.done {
        task_done(database_name, done_id)?;
    } else if u_arg.list {
        list_all_tasks(database_name)?;
    }

    Ok(())
}

fn add_task(db_name: &str, new_task: &String) -> Result<(), Box<dyn Error>> {
    let mut db = if db_exists(db_name) {
        PickleDb::load(db_name, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?
    } else {
        PickleDb::new(db_name, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)
    };

    let data = new_task.split(":").collect::<Vec<&str>>();

    if data.len() == 2 {
        db.set(data[0], &data[1].to_string())?;

        println!("The value of id: {} is {}", data[0], db.get::<String>(data[0]).unwrap());
    } else {
        println!("Data should be in 'id:value' format");
    }
    
    Ok(())
}

fn task_done(db_name: &str, old_task: usize) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let mut db = PickleDb::load(db_name, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?;

        let success = db.rem(&old_task.to_string())?;
    
        if success {
            println!("Task is marked done successfully");
        } else {
            println!("Task is not found");
        }
    } else {
        println!("Database doesn't exist");
    };

    Ok(())
}

fn list_all_tasks(db_name: &str) -> Result<(), Box<dyn Error>> {
    if db_exists(db_name) {
        let mut db = PickleDb::load(db_name, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?;

        println!("You have the following tasks: ");
        for kv in db.iter() {
            println!("{}. {}", kv.get_key(), kv.get_value::<String>().unwrap());
        }
    } else {
        println!("Database doesn't exist");
    };

    Ok(())
}

fn db_exists(db_name: &str) -> bool {
    Path::new(db_name).exists()
}
