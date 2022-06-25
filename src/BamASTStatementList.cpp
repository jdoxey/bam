#include "BamASTStatementList.h"

#include <iostream>

using namespace std;

void BamASTStatementList::print() {
    if (this->statementList) {
        this->statementList->print();
    }
    this->statement->print();
    cout << endl;
}
