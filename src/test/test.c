#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#ifdef _WIN32
#define popen _popen
#define pclose _pclose
#endif

void testFileShouldOutput(char *fileName, char *expectedOutput);

int main() {
	testFileShouldOutput("identifier.bam", "IDENTIFIER: myvariable\n");
	testFileShouldOutput("identifier-newline.bam", "IDENTIFIER: myvariable\n");
	testFileShouldOutput("identifier-semicolon.bam", "IDENTIFIER: myvariable\n");
	testFileShouldOutput("identifier-semicolon-newline.bam", "IDENTIFIER: myvariable\n");

	testFileShouldOutput("identifier-semicolon-identifier.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-semicolon-identifier-newline.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-semicolon-identifier-semicolon.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-semicolon-identifier-semicolon-newline.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");

	testFileShouldOutput("identifier-newline-identifier.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-newline-identifier-newline.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-newline-identifier-semicolon.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
	testFileShouldOutput("identifier-newline-identifier-semicolon-newline.bam", "IDENTIFIER: myvariable\nIDENTIFIER: othervariable\n");
}

void testFileShouldOutput(char *fileName, char *expectedOutput) {
#ifdef _WIN32
	char *commandLineFormatString = "type src\\test\\examples\\%s | build-files\\bam";
#else
	char *commandLineFormatString = "cat src/test/examples/%s | build-files/bam";
#endif
	char commandLine[1024];
	sprintf(commandLine, commandLineFormatString, fileName);

	FILE *outputPipe;
	char actualOutput[1024] = { 0 };
	outputPipe = popen(commandLine, "r");
	fread(actualOutput, sizeof(actualOutput), 1, outputPipe);
	pclose(outputPipe);

	if (strcmp(actualOutput, expectedOutput) != 0) {
		puts("TEST FAILED!");
		printf("Parsing file '%s', was expecting '%s', but got '%s'\n",
				fileName, expectedOutput, actualOutput);
		exit(1);
	}
}
