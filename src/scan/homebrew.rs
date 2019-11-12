use super::CommandVersion;
use glob::glob;
use std::path::*;

const HOMEBREW_GLOB: &str = "/usr/local/opt/*@*/bin/*";
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
        },
        _ => panic!(),
    };

    (command, version)
}

fn parse_version_path(path: PathBuf) -> CommandVersion {
    let (command, version) = extract_command_and_version(&path);

    CommandVersion { command, version, path }
}

pub fn scan(command: &str) -> Vec<CommandVersion> {
    let homebrew_glob = glob(HOMEBREW_GLOB).unwrap();
    let linuxbrew_glob = glob(LINUXBREW_GLOB).unwrap();
    let paths = homebrew_glob.chain(linuxbrew_glob);

    paths
        .flatten()
        .map(parse_version_path)
        .filter(|c| c.command == command)
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_path_node() {
        assert_eq!(
            parse_version_path(PathBuf::from("/usr/local/opt/node@8/bin/node")),
            CommandVersion {
                command: "node".to_string(),
                version: "8".to_string(),
                path: PathBuf::from("/usr/local/opt/node@8/bin/node"),
            }
        );
    }

    #[test]
    fn test_parse_version_path_php() {
        assert_eq!(
            parse_version_path(PathBuf::from("/usr/local/opt/php@5.6/bin/php")),
            CommandVersion {
                command: "php".to_string(),
                version: "5.6".to_string(),
                path: PathBuf::from("/usr/local/opt/php@5.6/bin/php"),
            }
        );
    }

    #[test]
    fn test_parse_linuxbrew_path_node() {
        assert_eq!(
            parse_version_path(PathBuf::from("/home/linuxbrew/.linuxbrew/opt/node@8/bin/node")),
            CommandVersion {
                command: "node".to_string(),
                version: "8".to_string(),
                path: PathBuf::from("/home/linuxbrew/.linuxbrew/opt/node@8/bin/node"),
            }
        );
    }
}
