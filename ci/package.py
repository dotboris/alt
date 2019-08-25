#!/usr/bin/env python3
import subprocess
from argparse import ArgumentParser
from os import path, makedirs
from shutil import copy, move, rmtree
from tempfile import mkdtemp


def step(message):
    print("\033[1m>>> {0}\033[0m".format(message))


def sh(*args):
    return subprocess.run(args, check=True)


def sh_capture(*args):
    return subprocess.check_output(args)


def build_release(rust_target):
    step('Building release build for target {}'.format(rust_target))
    sh('cargo', 'build', '--release', '--target', rust_target)


def build_gzip_bin(bin_path, version, rust_target, dest_dir):
    step('Packinging {0} as a gzipped binary'.format(bin_path))
    work_dir = mkdtemp()

    to_gzip_file = path.join(work_dir, 'alt')
    copy(bin_path, to_gzip_file)
    sh('gzip', '-fk9', to_gzip_file)
    move(
        '{}.gz'.format(to_gzip_file),
        path.join(dest_dir, 'alt_v{0}_{1}.gz'.format(version, rust_target))
    )

    rmtree(work_dir)


def list_output(dest_dir):
    step('Contents of {}'.format(dest_dir))
    sh('ls', '-lh', dest_dir)


def parse():
    parser = ArgumentParser()
    parser.add_argument('--dest-dir', required=True)
    parser.add_argument('--rust-target', required=True)
    return parser.parse_args()


def main(
    dest_dir=None,
    rust_target=None,
):
    step('Emptying {}'.format(dest_dir))
    if (path.exists(dest_dir)):
        rmtree(dest_dir)
    makedirs(dest_dir)

    build_release(rust_target)
    alt_bin = path.join('target', rust_target, 'release/alt')

    step('Looking up version')
    version = sh_capture(alt_bin, '--version')
    version = version.decode()
    version = version.strip().split(' ')[1]
    print(version)

    build_gzip_bin(alt_bin, version, rust_target, dest_dir)

    list_output(dest_dir)


if __name__ == "__main__":
    args = parse()
    main(**vars(args))
