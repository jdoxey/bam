#include "BamASTStatement.h"

#include <stdlib.h>

struct BamASTStatement *BamASTStatement_newIdentifier(struct BamASTIdentifier *identifier) {
    struct BamASTStatement *newStatement = malloc(sizeof(struct BamASTStatement));
    newStatement->type = BamASTStatementType_IDENTIFIER;
    newStatement->data = identifier;
    return newStatement;
}
