use std::{fs::{self, DirEntry}, io::{self, Write}};

pub fn get_pid(process_name: &str) -> Result<Vec<String>, String>{
    let proc_contents = fs::read_dir("/proc");
    let proc_contents = match proc_contents {
        Ok(dirs) => dirs,
        _ => return Err(String::from("Could not access /proc."))
    };

    let processes = proc_contents.filter(is_process);
    let processes_with_matching_name = processes.filter(|x| process_name_contains(x, process_name));
    let correct_pids = processes_with_matching_name.map(|x| x.unwrap()) //We checked for Ok() twice now in the filters.
                                                   .map(get_filename_str)
                                                   .collect();
    Ok(correct_pids)
    
}

fn get_filename_str(file: DirEntry) -> String {
    let filepath = file.file_name();
    let filename = filepath.to_str();
    String::from(filename.unwrap())
}

fn process_name_contains(process: &Result<DirEntry, io::Error>, compared_string: &str) -> bool{
    match process{
        Ok(dir) => {
            let pid = dir.file_name();
            let pid = pid.to_str();
            match pid {
                Some(current_pid) => {
                    let p_name = fs::read_to_string(format!("/proc/{}/comm", current_pid)).unwrap();
                    p_name.contains(compared_string)
                }
                _ => false,
            }
        },
        _ => false,
    }
}

fn is_process(entry: &Result<DirEntry, io::Error>) -> bool {
    match entry {
        Ok(x) => {
            let name = x.file_name();
            let name = name.to_str();
            match name {
                Some(s) => s.parse::<i32>().is_ok(),
                _ => false,
            }
        },
        _ => false,
    }
}

pub fn get_process_maps(pid: &str) -> Result<String, io::Error> {
    let path = format!("/proc/{}/maps", pid);
    fs::read_to_string(path)
}

pub fn write_to_file(path: &str, contents: Vec<u8>) -> Result<(), io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents.as_slice())?;
    Ok(())
}
