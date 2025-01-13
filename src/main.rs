use chrono::{DateTime, Days, Local};
use clap::Parser;
use std::fs::{self, File};
use std::io::Write;

#[derive(Parser)]
struct Cli {
    entry: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.entry.as_str() {
        "today" => new_entry_today()?,
        "tomorrow" => new_entry_tomorrow()?,
        _ => println!("Invalid entry."),
    }

    Ok(())
}

fn create_entry(daytime: DateTime<Local>) -> std::io::Result<()> {
    // let daytime: DateTime<Local> = Local::now();
    let doc_dir = dirs::document_dir().unwrap();
    let rel_dir = "Journal";
    let subpath = format!("{}", daytime.format(r"%Y/%m%b"));
    let filename = format!("{}", daytime.format(r"%Y-%m-%d.typ"));
    let dir = doc_dir.join(&rel_dir);
    let path = dir.join(&subpath);
    let file = path.join(&filename);

    fs::create_dir_all(path)?;
    // let mut file = match File::create_new(file) {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut file = File::create_new(file).expect("Entry already exists.");

    let template = "#import \"@local/journal:0.1.0\": journal\n\
        #let date = datetime(year: %Y, month: %m, day: %d)\n\
        #show: journal.with(date)";
    let template = format!("{}", daytime.format(template));

    file.write(template.as_bytes())?;

    println!("Created entry {}", filename);
    Ok(())
}

fn new_entry_today() -> std::io::Result<()> {
    let today: DateTime<Local> = Local::now();
    create_entry(today)?;
    Ok(())
}

fn new_entry_tomorrow() -> std::io::Result<()> {
    let day = Days::new(1);
    let today: DateTime<Local> = Local::now();
    let tomorrow = today + day;
    create_entry(tomorrow)?;
    Ok(())
}
