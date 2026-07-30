use ck_diff::{DiffLineKind, DiffResult, FileDiff};

pub fn print(diff: &DiffResult) {
    for file in &diff.files {
        print_file(file);
    }
}

fn print_file(file: &FileDiff) {
    println!("{}", file.path.display());

    for line in &file.lines {
        match line.kind {
            DiffLineKind::Context => {
                println!(" {}", line.text);
            }

            DiffLineKind::Added => {
                println!("+{}", line.text);
            }

            DiffLineKind::Removed => {
                println!("-{}", line.text);
            }
        }
    }

    println!();
}
