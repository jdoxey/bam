#include "BamASTFunctionCall.h"

#include <iostream>

using namespace std;

void BamASTFunctionCall::print() {
    cout << "{\"type\":\"BamFunctionCall\",\"name\":\"" << this->functionName << "\",";
    cout << "\"arguments\":[";
    if (this->argument != nullptr) {
        this->argument->print();
    }
    cout << "]";
    cout << "}";
}
