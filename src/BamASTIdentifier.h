#ifndef BAM_AST_IDENTIFIER_H
#define BAM_AST_IDENTIFIER_H

struct BamASTIdentifier {
    char *name;
};

struct BamASTIdentifier *BamASTIdentifier_new(char *name);

#endif  // BAM_AST_IDENTIFIER_H
