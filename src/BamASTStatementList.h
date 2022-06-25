#ifndef __BAMASTSTATEMENTLIST_H__
#define __BAMASTSTATEMENTLIST_H__


#include "BamASTStatement.h"

class BamASTStatementList {
    BamASTStatementList *statementList;
    BamASTStatement *statement;
public:
    BamASTStatementList(BamASTStatementList *statementList, BamASTStatement *statement) :
        statementList(statementList),
        statement(statement)
        {}
    void print();
};


#endif  // __BAMASTSTATEMENTLIST_H__
