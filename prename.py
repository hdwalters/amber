#!/usr/bin/env python3
import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

def read_mappings() -> dict[str, tuple[str, str]]:
    mappings = dict()
    visited = set()
    path = Path(__file__).with_suffix(".txt")
    with open(path, 'r') as reader:
        for line in reader:
            if match := re.search(r'^(\w+)\.ab\s+(\w+)\s+(\w+)\b', line):
                lib, old, new = match.groups()
                if new in visited:
                    raise RuntimeError(f'Duplicate mappings: "{new}"')
                prefix = f'{lib}_'
                mappings[old] = (prefix, new)
                visited.add(new)
    return mappings

def not_empty(line: str) -> bool:
    return bool(re.search(r'.', line))

def modify_line(line: str, mappings: dict[str, tuple[str, str]]) -> tuple[str, bool]:
    if re.search(r'^(.*\bimport\s+\*\s+from\b.+)$', line):
        return line, True
    elif match := re.search(r'^(.*\bimport\s+\{)(.+)(}\s+from\b.+)$', line):
        prefix, body, suffix = match.groups()
        for old, (_, new) in mappings.items():
            if new != old:
                body = re.sub(rf'\b{old}\b', new, body)
        return f'{prefix}{body}{suffix}\n', True
    else:
        for old, (_, new) in mappings.items():
            if new != old:
                line = re.sub(rf'\b{old}\(', f'{new}(', line)
        return line, False

def modify_stream(reader, writer, mappings: dict[str, tuple[str, str]]) -> bool:
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

def modify_file(path: Path, mappings: dict[str, tuple[str, str]], dryrun: bool):
    temp = path.with_suffix('.tmp')
    with open(path, 'r') as reader:
        if dryrun:
            dirty = modify_stream(reader, None, mappings)
        else:
            with open(temp, 'w') as writer:
                dirty = modify_stream(reader, writer, mappings)
    if dirty:
        if dryrun:
            print(f'[dryrun] Modifying: {path}')
        else:
            print(f'Modifying: {path}')
            stat = path.stat()
            temp.chmod(stat.st_mode)
            temp.rename(path)
    elif not dryrun:
        temp.unlink()

def rename_stem(old: str, mappings: dict[str, tuple[str, str]]) -> str|None:
    if pair := mappings.get(old):
        prefix, new = pair
        if new.startswith(prefix):
            return new
        else:
            return f'{prefix}{new}'
    return None

def rename_file(path: Path, mappings: dict[str, tuple[str, str]], dryrun: bool):
    old = path.stem
    if new := rename_stem(old, mappings):
        if new != old and not re.search(r'\b(validity|no_output)\b', str(path)):
            move = path.with_stem(new)
            if move.exists():
                raise RuntimeError(f'Already exists: {move}')
            if dryrun:
                print(f'[dryrun] Renaming: {path} as {move}')
            else:
                print(f'Renaming: {path} as {move}')
                command = ['git', 'mv', path, move]
                subprocess.run(command)
        elif dryrun:
            print(f'[dryrun] Skipping: {path}')
    elif dryrun:
        print(f'[dryrun] Skipping: {path}')

def walk_dir(mappings, settings):
    parent = Path(__file__).parent
    for rootdir, subdirs, filenames in os.walk(parent):
        rootdir = Path(rootdir).relative_to(parent)
        subdirs.sort()
        filenames.sort()
        for filename in filenames:
            path = rootdir / filename
            if path.suffix == '.ab':
                if settings.modify:
                    modify_file(path, mappings, settings.dryrun)
                if settings.rename:
                    rename_file(path, mappings, settings.dryrun)

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-m', '--modify', action='store_true', help='rename stdlib functions')
    parser.add_argument('-r', '--rename', action='store_true', help='rename script files')
    parser.add_argument('-d', '--dryrun', action='store_true', help='do not make changes')
    return parser.parse_args()

def run_main():
    settings = parse_args()
    if settings.modify or settings.rename:
        mappings = read_mappings()
        walk_dir(mappings, settings)

try:
    run_main()
except (IOError, RuntimeError, KeyboardInterrupt) as error:
    print(error, file=sys.stderr)
