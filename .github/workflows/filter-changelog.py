from sys import stdin, argv

version = argv[1]
lines = []
match_version = f"## [{version}]"
begun = False

for line in map(str.rstrip, stdin):
    if line.startswith(match_version):
        begun = True
        continue
    if begun and line.startswith("## [v"):
        break
    if begun:
        lines.append(line)

print("\n".join(lines).rstrip())
