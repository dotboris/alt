#!/usr/bin/env python3
import json
from pathlib import Path
import subprocess
from subprocess import PIPE


def main():
    root = Path(__file__).parent.parent
    metadata_process = subprocess.run(
        ["cargo", "metadata", "--format-version=1", "--no-deps"],
        cwd=root,
        stdout=PIPE,
        check=True,
    )
    metadata = json.loads(metadata_process.stdout)

    packages = [package for package in metadata["packages"] if package["name"] == "alt"]
    assert len(packages) == 1, "Expected exactly 1 package named alt"

    package = packages[0]
    print(f"v{package['version']}", end="")


if __name__ == "__main__":
    main()
