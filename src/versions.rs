use std::path::PathBuf;

const FILE_NAME: &'static str = ".alt.yml";

pub fn find_file(mut dir: PathBuf) -> Option<PathBuf> {
    loop {
        let file = dir.join(FILE_NAME);

        if file.is_file() {
            return Some(file);
        } else {
            if dir.parent().is_none() {
                return None;
            } else {
                dir.pop();
            }
        }
    }
}
