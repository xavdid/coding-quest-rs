use regex::Regex;
use std::collections::HashMap;

fn is_marked_for_deletion(name: &str) -> bool {
    name.contains("temporary") || name.contains("delete")
}

#[derive(Debug)]
struct FileOrFolder {
    size_or_folder_id: u64,
    is_folder: bool,
    should_delete: bool,
}

fn size_to_delete(
    folders: &HashMap<u64, Vec<FileOrFolder>>,
    folder_id: &u64,
    delete_all: bool,
) -> u64 {
    folders
        .get(folder_id)
        .unwrap_or_else(|| panic!("unable to find {folder_id} in folders"))
        .iter()
        .map(|f| {
            let delete = f.should_delete || delete_all;
            if f.is_folder {
                size_to_delete(folders, &f.size_or_folder_id, delete)
            } else if delete {
                f.size_or_folder_id
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let folder_tag = Regex::new(r"\[FOLDER (\d+)\]").unwrap();

    let folders: HashMap<u64, Vec<FileOrFolder>> = include_str!("input-full.txt")
        .split("Folder: ")
        // empty bit at the start of the file
        .skip(1)
        .map(|f| {
            let mut lines = f.lines();
            let num: u64 = str::parse(lines.next().unwrap()).unwrap();

            let items: Vec<FileOrFolder> = lines
                .map(|l| {
                    let name = l.trim().split_ascii_whitespace().nth(1).unwrap();
                    let should_delete = is_marked_for_deletion(name);

                    if let Some(folder_info) = folder_tag.captures(l) {
                        FileOrFolder {
                            should_delete,
                            is_folder: true,
                            size_or_folder_id: str::parse(&folder_info[1]).unwrap(),
                        }
                    } else {
                        FileOrFolder {
                            should_delete,
                            is_folder: false,
                            size_or_folder_id: str::parse(
                                l.trim().split_ascii_whitespace().nth(2).unwrap(),
                            )
                            .unwrap(),
                        }
                    }
                })
                .collect();

            (num, items)
        })
        // .map(|l| l.trim()).collect()})
        .collect();

    // dbg!(folders);
    let result = size_to_delete(&folders, &0, false);
    // assert_eq!(result, 103879262); // demo data
    assert_eq!(result, 349035592144); // real data
    println!("result: {result}");
}
