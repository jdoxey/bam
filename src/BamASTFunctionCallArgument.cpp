#include "BamASTFunctionCallArgument.h"

#include <iostream>

using namespace std;

void BamASTFunctionCallArgument::print() {
    cout << "{\"type\":\"BamFunctionCallArgument\",\"name\":\"" << this->name << "\",\"value\":";
    this->valueExpression->print();
    cout << "}";
}
