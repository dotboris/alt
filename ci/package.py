#!/usr/bin/env python3
import subprocess
import sys
from argparse import ArgumentParser
from os import path, makedirs
from shutil import copy, copytree, move, rmtree, which
from tempfile import mkdtemp


def step(message):
    print("\033[1m>>> {0}\033[0m".format(message))


def sh(*args):
    return subprocess.run(args, check=True)


def sh_capture(*args):
    return subprocess.check_output(args)


def install(src, dest, mode):
    return sh('install', '-D', '-m', mode, src, dest)


def command_exists(command):
    return which(command) is not None


def is_platform(platform):
    return sys.platform.startswith(platform)


def build_release(rust_target):
    step('Building release build for target {}'.format(rust_target))
    sh('cargo', 'build', '--release', '--locked', '--target', rust_target)


def build_deb(rust_target, dest_dir):
    if not command_exists('cargo-deb'):
        step('Installing `cargo-deb` tool')
        sh('cargo', 'install', 'cargo-deb')

    step('Building deb package for {}'.format(rust_target))
    sh('cargo', 'deb', '--target', rust_target, '-o', dest_dir, '--', '--locked')


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


def build_tarbal(bin_path, version, rust_target, dest_dir):
    dest_file_name = 'alt_v{0}_{1}.tar.gz'.format(version, rust_target)
    step('Packinging {}'.format(dest_file_name))
    work_dir = mkdtemp()

    install(bin_path, path.join(work_dir, 'bin/alt'), '755')
    install('./README.md', path.join(work_dir, 'README.md'), '644')
    install('./LICENSE', path.join(work_dir, 'LICENSE'), '644')
    install('./etc/profile.d/alt.sh', path.join(work_dir, 'etc/profile.d/alt.sh'), '644')
    install('./etc/fish/conf.d/alt.fish', path.join(work_dir, 'etc/fish/conf.d/alt.fish'), '644')

    dest_file = path.join(dest_dir, dest_file_name)
    sh(
        'tar', '--owner=0', '--group=0',
        '-czf', path.abspath(dest_file),
        '-C', work_dir,
        '.'
    )

    sh('tar', 'tvzf', dest_file)

    rmtree(work_dir)


def list_output(dest_dir):
    step('Contents of {}'.format(dest_dir))
    sh('ls', '-lh', dest_dir)


def parse():
    parser = ArgumentParser()
    parser.add_argument('--dest-dir', required=True)
    parser.add_argument('--rust-target', required=True)
    parser.add_argument('--lazy-build', action='store_true')
    return parser.parse_args()


def main(
    dest_dir=None,
    rust_target=None,
    lazy_build=None
):
    step('Emptying {}'.format(dest_dir))
    if (path.exists(dest_dir)):
        rmtree(dest_dir)
    makedirs(dest_dir)

    alt_bin = path.join('target', rust_target, 'release/alt')
    if lazy_build and path.exists(alt_bin):
        step('Release {} already built, skipping because of --lazy-build'.format(alt_bin))
    else:
        build_release(rust_target)

    step('Looking up version')
    version = sh_capture(alt_bin, '--version')
    version = version.decode()
    version = version.strip().split(' ')[1]
    print(version)

    build_tarbal(alt_bin, version, rust_target, dest_dir)

    build_gzip_bin(alt_bin, version, rust_target, dest_dir)

    if is_platform('linux'):
        build_deb(rust_target, dest_dir)

    list_output(dest_dir)


if __name__ == "__main__":
    args = parse()
    main(**vars(args))
