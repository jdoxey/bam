# Roadmap

## Versions 0.1
- An end-user can (on a base install of their OS, i.e. no compiler components installed),
    - download bam-0.1.0.zip (or similar) from GitHub releases,
    - unzip it somewhere and update their path,
    - create a file, e.g. hello.bam
    - run `bam hello.bam` to compile it
    - run the resulting executable, e.g. `hello` and see the output in the terminal/command prompt
- This should work on Linux, macOS and Windows
- Language only has enough implemented to print to the console
- GitHub Actions compiles, tests, packages and deploys the new version separately for each platform
