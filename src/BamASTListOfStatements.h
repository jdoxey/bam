#ifndef BAM_AST_LIST_OF_STATEMENTS_H
#define BAM_AST_LIST_OF_STATEMENTS_H

#include "BamASTStatement.h"

struct BamASTListOfStatements {
    struct BamASTListOfStatements *firstStatements;
    struct BamASTStatement *lastStatement;
};

struct BamASTListOfStatements *BamASTListOfStatements_new(struct BamASTListOfStatements *firstStatements, struct BamASTStatement *last);
void BamASTListOfStatements_print(struct BamASTListOfStatements *statements);

#endif  // BAM_AST_LIST_OF_STATEMENTS_H
