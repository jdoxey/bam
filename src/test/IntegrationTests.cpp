#include <string>
#include <iostream>
#include <cstdio>
#include <memory>
#include <stdexcept>
#include <array>

using namespace std;

string exec(const char* cmd) {
    array<char, 128> buffer;
    string result;
    unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
    if (!pipe) {
        throw runtime_error("popen() failed!");
    }
    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
        result += buffer.data();
    }
    return result;
}

void expectOutput(string testFileName, string expectedJSONOutput) {
    string commandLine = "cat src/test/examples/" + testFileName +
            " | build-files/bam";
    string actualJSONOutput = exec(commandLine.c_str());
    if (!(actualJSONOutput == expectedJSONOutput)) {
        cout << "TEST FAILED" << endl;
        cout << "   Testing file: " << testFileName << endl;
        cout << "   Actual output was: " << actualJSONOutput << endl;
        cout << "        but expected: " << expectedJSONOutput << endl;
        throw exception();
    }
}

int main(int argc, char *argv[]) {
    try {
        expectOutput("function-call-print+string.bam",
                "{\"type\":\"BamFunctionCall\",\"name\":\"print\",\"arguments\":[{\"type\":\"BamFunctionCallArgument\",\"name\":\"message\",\"value\":\"Hello world\"}]}\n");
        return 0;
    }
    catch (exception) {
        return 1;
    }
}
