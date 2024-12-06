#!/usr/bin/env python3
import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

def read_mappings(script: Path) -> dict[str, str]:
    mappings = dict()
    visited = set()
    path = script.with_suffix(".txt")
    with open(path, 'r') as reader:
        for line in reader:
            if match := re.search(r'^\w+\.ab\s+(\w+)\s+(\w+)(?:\s+(\S+))?$', line):
                old, new, flag = match.groups()
                if not flag:
                    raise RuntimeError(f'Missing flag: "{old}" to "{new}"')
                if new != old:
                    if flag != 'Yes':
                        raise RuntimeError(f'Unexpected flag: "{flag}" for "{old}" to "{new}"')
                    if new in visited:
                        raise RuntimeError(f'Duplicate mappings: "{new}"')
                    mappings[old] = new
                    visited.add(new)
                else:
                    if flag != '-':
                        raise RuntimeError(f'Unexpected flag: "{flag}" for "{old}" to "{new}"')
    return mappings

def not_empty(line):
    return bool(re.search(r'.', line))

def modify_line(line, mappings: dict[str, str]):
    if re.search(r'^(.*\bimport\s+\*\s+from\b.+)$', line):
        return line, True
    elif match := re.search(r'^(.*\bimport\s+\{)(.+)(}\s+from\b.+)$', line):
        prefix, body, suffix = match.groups()
        for old, new in mappings.items():
            body = re.sub(rf'\b{old}\b', new, body)
        return f'{prefix}{body}{suffix}\n', True
    else:
        for old, new in mappings.items():
            line = re.sub(rf'\b{old}\(', f'{new}(', line)
        return line, False

def modify_stream(reader, writer, mappings: dict[str, str]) -> bool:
    dirty = False
    imported = False
    for line in reader:
        modified, importing = modify_line(line, mappings)
        if imported and not importing and not_empty(line):
            if writer:
                print('', file=writer)
            dirty = True
        imported = importing
        if writer:
            print(modified, end='', file=writer)
        if modified != line:
            dirty = True
    return dirty

def modify_file(path: Path, mappings: dict[str, str], dryrun: bool):
    temp = path.with_suffix('.tmp')
    with open(path, 'r') as reader:
        if not dryrun:
            with open(temp, 'w') as writer:
                dirty = modify_stream(reader, writer, mappings)
        else:
            dirty = modify_stream(reader, None, mappings)
    if dirty:
        if not dryrun:
            print(f'Modifying: {path}')
            stat = path.stat()
            temp.chmod(stat.st_mode)
            temp.rename(path)
        else:
            print(f'[dryrun] Modifying: {path}')
    else:
        if not dryrun:
            temp.unlink()

def find_prefix(path: Path, mappings: dict[str, str], dryrun: bool):
    stem = path.stem
    if new := mappings.get(stem):
        return new
    if dryrun:
        for old, new in sorted(mappings.items()):
            if stem.startswith(old):
                print(f'[dryrun] Skipping: {path} "{old}" to "{new}"')
    return None

def rename_file(path: Path, mappings: dict[str, str], dryrun: bool):
    if new := find_prefix(path, mappings, dryrun):
        move = path.with_stem(new)
        if move.exists():
            raise RuntimeError(f'Already exists: {move}')
        if not dryrun:
            print(f'Renaming: {path} to {move}')
            command = ['git', 'mv', path, move]
            subprocess.run(command)
        else:
            print(f'[dryrun] Renaming: {path} to {move}')

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-m', '--modify', action='store_true', help='rename stdlib functions')
    parser.add_argument('-r', '--rename', action='store_true', help='rename script files')
    parser.add_argument('-d', '--dryrun', action='store_true', help='do not make changes')
    return parser.parse_args()

def run_main():
    settings = parse_args()
    script = Path(__file__)
    parent = script.parent
    mappings = read_mappings(script)
    for rootdir, subdirs, filenames in os.walk(parent):
        subdirs.sort()
        filenames.sort()
        for filename in filenames:
            path = Path(rootdir)
            path = path.relative_to(parent) / filename
            if path.suffix == '.ab':
                if settings.modify:
                    modify_file(path, mappings, settings.dryrun)
                if settings.rename:
                    rename_file(path, mappings, settings.dryrun)

try:
    run_main()
except (IOError, RuntimeError, KeyboardInterrupt) as error:
    print(error, file=sys.stderr)
