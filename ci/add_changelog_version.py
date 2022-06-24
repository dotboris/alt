#!/usr/bin/env python3
from argparse import ArgumentParser
from datetime import date
from enum import Enum
from pathlib import Path
import re

REPO_URL = "https://github.com/dotboris/alt"


class Section(Enum):
    PREAMBLE = "preamble"
    UNRELEASED_BODY = "unreleased-body"
    PREVIOUS_RELEASES = "previous-releases"
    REFS = "refs"


def parse_changelog(changelog_path: Path) -> dict[Section, list[str]]:
    sections = {location: [] for location in Section}

    location = Section.PREAMBLE
    with changelog_path.open("r") as fh:
        for line in fh.readlines():
            if match := re.match(r"<!-- section:(?P<section>.*) -->", line):
                section = match.group("section")
                location = Section(section)

            sections[location].append(line)

    return sections


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("--old-version", required=True)
    parser.add_argument("--new-version", required=True)
    return parser.parse_args()


def main(*, old_version, new_version):
    root = Path(__file__).parent.parent
    changelog_path = root / "CHANGELOG.md"

    sections = parse_changelog(changelog_path)

    today = date.today()
    new_changelog = [
        *sections[Section.PREAMBLE],
        "<!-- section:unreleased-body -->\n\n",
        "<!-- section:previous-releases -->\n"
        f"## [{new_version}] {today.isoformat()}\n",
        *sections[Section.UNRELEASED_BODY][1:],
        *sections[Section.PREVIOUS_RELEASES][1:],
        "<!-- section:refs -->\n",
        f"[{new_version}]: {REPO_URL}/compare/{old_version}..{new_version}\n",
        *sections[Section.REFS][1:],
    ]

    with changelog_path.open("w") as fh:
        fh.writelines(new_changelog)

    # This is a multiline string and needs to be quoted. This looks like full
    # blown URL encoding but it actually only supports the %, \n and \r
    # characters.
    # See: https://github.community/t/set-output-truncates-multiline-strings/16852/3
    unreleased_body = "".join(sections[Section.UNRELEASED_BODY][1:]).strip()
    unreleased_body = unreleased_body.replace("%", "%25")
    unreleased_body = unreleased_body.replace("\n", "%0A")
    unreleased_body = unreleased_body.replace("\r", "%0D")
    print(f"::set-output name=unreleasedBody::{unreleased_body}")


if __name__ == "__main__":
    args = parse_args()
    main(**vars(args))
