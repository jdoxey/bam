#include "BamASTListOfStatements.h"

#include <stdlib.h>  // malloc
#include <stdio.h>  // printf

#include "BamASTIdentifier.h"

struct BamASTListOfStatements *BamASTListOfStatements_new(struct BamASTListOfStatements *firstStatements, struct BamASTStatement *lastStatement) {
    struct BamASTListOfStatements *newList = malloc(sizeof(struct BamASTListOfStatements));
    newList->firstStatements = firstStatements;
    newList->lastStatement = lastStatement;
    return newList;
}

void BamASTListOfStatements_print(struct BamASTListOfStatements *statements) {    
    if (statements->firstStatements != NULL) {
        BamASTListOfStatements_print(statements->firstStatements);
    }
    struct BamASTIdentifier *identifier = NULL;
    switch (statements->lastStatement->type) {
        case BamASTStatementType_IDENTIFIER:
            identifier = (struct BamASTIdentifier *)statements->lastStatement->data;
            printf("IDENTIFIER: %s\n", identifier->name);
    }
}
