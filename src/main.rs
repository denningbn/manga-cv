use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;



fn main() {
    let mut title = String::new();
    
    let mut args: Vec<String> = env::args().collect();

    let mut prefix = String::from("Vol");
    let mut path = String::from(".");
    let mut offset = 0;
    let mut help_arg = false;

    args.remove(0);

    process_args(&args, &mut prefix, &mut title, &mut path, &mut offset, &mut help_arg);

    let path2 = Path::new(&path);

    if !path2.exists()
    {
        path = String::from(".");
    }


    if !help_arg
    {
        check_output_dir(&path);
        traverse_path(path, title, prefix, offset );
    }

}


//process_args
//Usage: takes all of the arguments given to manga_cv and parses them.

fn process_args(args: &[String], prefix: &mut String, title: &mut String, path: &mut String, offset: &mut u32, help_arg: &mut bool)
{
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next()
    {
        match arg.as_str() 
        {
            "-t" => 
            {
                if let Some(peeked_val) = iter.peek().cloned()
                {
                    *title = peeked_val.to_string();
                }
                
            },

            "-c" =>
            {
                *prefix = String::from("Ch");
            },

            "-h" => 
            {
                print_usage();
                *help_arg = true;
            },

            "-o"=>
            {
                if let Some(peeked_val) = iter.peek().cloned()
                {

                    match peeked_val.to_string().parse::<u32>() {
                        Ok(value) => {
                            *offset = value;
                        }
                        Err(err) => {
                            eprintln!("Error converting to u32: {}", err);
                        }
                    }
                }
            },

            _ =>{},
        }
    }

    if let Some(last_arg) = args.last().cloned()
    {
        *path = last_arg;
    }
}

fn print_usage()
{
    println!("usage: manga-cv [Options] [DIRECTORY]");

    println!("  -o");
    println!("      offset, determines what number the program starts incrementing as");
    println!("  -t");
    println!("      title, determines what the output is named");
}


fn traverse_path(path: String, title: String, prefix: String, offset: u32)
{
    let mut i = 0;

     if let Ok(entries) = fs::read_dir(path) {

            let mut entries: Vec<_> = entries.collect();
            entries.sort_by(|a, b| {
                a.as_ref()
                    .unwrap()
                    .file_name()
                    .cmp(&b.as_ref().unwrap().file_name())
            });

            for entry in entries {
                if let Ok(entry) = entry {
                    // Access information about the entry
                    let entry_path = entry.path();
                    let entry_file_name = entry.file_name();


                    let o = "output";
                    if entry_file_name != o
                    {
                        execute_once(entry_path, offset + i, &title, prefix.clone());
                        i = i + 1;
                    }
                }
            }
    } else {
        println!("Error reading directory");
    }
}

fn check_output_dir(_path: &str)
{
    let path = Path::new(&_path);
    let output_present = path.join("output").exists();

    
    if !output_present
    {
        make_output(_path);
    }
}


//creates an output directory to pull all output files in
fn make_output(path: &str)
{
    let output_suffix = "/output";
    let output_str = path.to_string() + output_suffix;

    let output_path = Path::new(&output_str);

    match fs::create_dir_all(output_path)
    {
        Ok(_) => 
        {
            println!("Output directory successfully created");
        },
        Err(err) => 
        {
            eprintln!("Error creating directory: {}", err);
        },
    }

}


fn execute_once(path: PathBuf, index: u32, title:&str, prefix: String)
{
    let mut command = Command::new("convert");

    let path = path.join("*");
    
    let title = format!("{}{}{}.pdf", title, prefix, index);
    
    let mut dest = path.clone();
    dest.pop();
    dest.pop();
    let dest = dest.join("output").join(title.clone());

    command.arg(path.to_str().unwrap());

    command.arg(dest.clone());

    
    println!("{}", title);
    println!("{}", path.display());
    println!("{}", dest.display());
    let result = command.output();

    match result
    {
        Ok(_) =>println!("Successfully converted to PDF\n"),
        Err(_)=>println!("Error in execution of Convert"),
    }
}
