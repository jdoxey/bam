# Roadmap

## Versions 0.1
- An end-user can (on a base install of their OS, i.e. no compiler components installed),
    - download bam-0.1.0.zip (or similar) from GitHub releases,
    - unzip it somewhere and update their path,
    - create a file, e.g. hello.bam
    - run `bam hello.bam` to compile and link it into the final executable
    - run the resulting executable, e.g. `hello` and see the output in the terminal/command prompt
- This should work on Linux, macOS and Windows
- Language only has enough implemented to print to the console
- GitHub Actions compiles, tests, packages and deploys the new version separately for each platform

### Remaining tasks for this version
- Make sure it runs on Windows and macOS
- Remove temporary files from release zip
- Make a containing folder inside the zip (nobody likes to unzip a zip file and have the files mix with everything in the current directory)
- Beta release always has '.1.'
- The Beta releases come at the top of the 'Releases' list in GitHub. This will make it harder for people to easily find the 'latest stable release'.
- Align jobs that get run in the different builds (make sure beta.yml, pr.yml and release.yml build processes are aligned).
- Fix Apple workaround for print statement (hard coded length)
