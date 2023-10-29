use glob::glob;
use std::path::*;

use crate::command_version::CommandVersion;

const HOMEBREW_GLOB: &str = "/usr/local/opt/*@*/bin/*";
const HOMEBREW_OPT_GLOB: &str = "/opt/homebrew/opt/*@*/bin/*";
const LINUXBREW_GLOB: &str = "/home/linuxbrew/.linuxbrew/opt/*@*/bin/*";

fn extract_command_and_version(path: &Path) -> (String, String) {
    let parts: Vec<_> = path.components().collect();
    let num_parts = parts.len();

    let command = match parts[num_parts - 1] {
        Component::Normal(part) => String::from(part.to_str().unwrap()),
        _ => panic!(),
    };
    let version = match parts[num_parts - 3] {
        Component::Normal(part) => {
            let package = part.to_str().unwrap();
            let version = package.split('@').last().unwrap();
            String::from(version)
        }
        _ => panic!(),
    };

    (command, version)
}

fn parse_version_path(path: PathBuf) -> CommandVersion {
    let (command_name, version_name) = extract_command_and_version(&path);

    CommandVersion {
        command_name,
        version_name,
        path,
    }
}

pub fn scan(command: &str) -> Vec<CommandVersion> {
    let homebrew_glob = glob(HOMEBREW_GLOB).unwrap();
    let homebrew_opt_glob = glob(HOMEBREW_OPT_GLOB).unwrap();
    let linuxbrew_glob = glob(LINUXBREW_GLOB).unwrap();
    let paths = homebrew_glob.chain(homebrew_opt_glob).chain(linuxbrew_glob);

    paths
        .flatten()
        .map(parse_version_path)
        .filter(|c| c.command_name == command)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_path_node() {
        assert_eq!(
            parse_version_path(PathBuf::from("/usr/local/opt/node@8/bin/node")),
            CommandVersion::new("node", "8", Path::new("/usr/local/opt/node@8/bin/node"),)
        );
    }

    #[test]
    fn test_parse_version_path_php() {
        assert_eq!(
            parse_version_path(PathBuf::from("/usr/local/opt/php@5.6/bin/php")),
            CommandVersion::new("php", "5.6", Path::new("/usr/local/opt/php@5.6/bin/php"))
        );
    }

    #[test]
    fn test_parse_opt_homebrew_path_node() {
        assert_eq!(
            parse_version_path(PathBuf::from("/opt/homebrew/opt/node@16/bin/node")),
            CommandVersion::new(
                "node",
                "16",
                Path::new("/opt/homebrew/opt/node@16/bin/node"),
            )
        );
    }

    #[test]
    fn test_parse_opt_homebrew_path_php() {
        assert_eq!(
            parse_version_path(PathBuf::from("/opt/homebrew/opt/php@8.2/bin/php")),
            CommandVersion::new("php", "8.2", Path::new("/opt/homebrew/opt/php@8.2/bin/php")),
        );
    }

    #[test]
    fn test_parse_linuxbrew_path_node() {
        assert_eq!(
            parse_version_path(PathBuf::from(
                "/home/linuxbrew/.linuxbrew/opt/node@8/bin/node"
            )),
            CommandVersion::new(
                "node",
                "8",
                Path::new("/home/linuxbrew/.linuxbrew/opt/node@8/bin/node"),
            )
        );
    }
}
