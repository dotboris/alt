#!/usr/bin/env python3
from argparse import ArgumentParser
from datetime import date
from pathlib import Path

REPO_URL = "https://github.com/dotboris/alt"


def parse():
    parser = ArgumentParser()
    parser.add_argument("--old-version", required=True)
    parser.add_argument("--new-version", required=True)
    return parser.parse_args()


def main(*, old_version, new_version):
    root = Path(__file__).parent.parent
    changelog_path = root / "CHANGELOG.md"

    new_changelog = []
    with changelog_path.open("r") as fh:
        for line in fh.readlines():
            new_changelog.append(line)

            if "<!-- release:new-version-section -->" in line:
                today = date.today()
                new_changelog.append(f"\n## [{new_version}] {today.isoformat()}\n")
            elif "<!-- release:new-version-ref -->" in line:
                new_changelog.append(
                    f"[{new_version}]: "
                    f"{REPO_URL}/compare/{old_version}..{new_version}\n"
                )

    with changelog_path.open("w") as fh:
        fh.writelines(new_changelog)


if __name__ == "__main__":
    args = parse()
    main(**vars(args))
