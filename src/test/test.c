#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

int main() {
    FILE *output_pipe;
    char output_string[1024];
#ifdef _WIN32
    output_pipe = _popen("type src\\test\\examples\\identifier.bam | build-files\\bam", "r");
#else
    output_pipe = popen("cat src/test/examples/identifier.bam | build-files/bam", "r");
#endif

    fgets(output_string, sizeof(output_string), output_pipe);
    assert(strcmp(output_string, "IDENTIFIER: myvariable\n") == 0);

#ifdef _WIN32
    _pclose(output_pipe);
#else
    pclose(output_pipe);
#endif
}
