#ifndef BAM_AST_STATEMENT_H
#define BAM_AST_STATEMENT_H

#include "BamASTIdentifier.h"

enum BamASTStatementType {
    BamASTStatementType_IDENTIFIER
};

struct BamASTStatement {
    enum BamASTStatementType type;
    void *data;
};

struct BamASTStatement *BamASTStatement_newIdentifier(struct BamASTIdentifier *identifier);

#endif  // BAM_AST_STATEMENT_H
