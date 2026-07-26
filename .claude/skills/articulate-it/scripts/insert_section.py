#!/usr/bin/env python3
"""Append/insert a drafted 'Articulate It' section into a target doc.

Usage: insert_section.py <section_file.md> <target_file.md>

Inserts the section immediately before a trailing nav footer (a line that is
exactly "---" within the last ~800 characters of the file, e.g. the
**Previous:**/**Next:** or **See also:** line pattern used across this repo's
tutorials). If no such footer is found, appends the section at end-of-file.

Idempotency: if the target file already contains an "## Articulate It:"
heading, the script refuses to insert a second one (use --replace to swap the
existing section out first).
"""
import sys

HEADING = "## Articulate It: Interview Framing & Vocabulary"


def main():
    args = sys.argv[1:]
    replace = "--replace" in args
    args = [a for a in args if a != "--replace"]
    if len(args) != 2:
        print(__doc__)
        sys.exit(1)
    section_path, target_path = args

    with open(section_path, encoding="utf-8") as f:
        section = f.read()
    if not section.lstrip().startswith(HEADING):
        print(f"WARNING: section file does not start with '{HEADING}' — "
              f"proceeding anyway, but check your draft.")
    if not section.endswith("\n\n"):
        section = section.rstrip("\n") + "\n\n"

    with open(target_path, encoding="utf-8") as f:
        text = f.read()

    if HEADING in text:
        if not replace:
            print(f"SKIPPED (already present): {target_path} "
                  f"— pass --replace to overwrite the existing section.")
            sys.exit(0)
        # Cut out the old section: from HEADING to the next trailing nav
        # footer marker, or to EOF if none.
        start = text.index(HEADING)
        rest = text[start + len(HEADING):]
        needle = "\n---\n"
        idx_in_rest = rest.rfind(needle)
        if idx_in_rest != -1 and (len(rest) - idx_in_rest) < 800:
            end = start + len(HEADING) + idx_in_rest + 1  # keep the "---\n"
            text = text[:start] + text[end:]
        else:
            text = text[:start].rstrip("\n") + "\n"

    needle = "\n---\n"
    idx = text.rfind(needle)
    if idx != -1 and (len(text) - idx) < 800:
        new_text = text[:idx] + "\n" + section + text[idx + 1:]
    else:
        new_text = text.rstrip("\n") + "\n\n" + section
        print(f"NOTE: no trailing nav footer found, appended at EOF: {target_path}")

    with open(target_path, "w", encoding="utf-8") as f:
        f.write(new_text)
    print(f"OK: {target_path}")


if __name__ == "__main__":
    main()
