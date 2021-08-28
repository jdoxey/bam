#include "BamASTIdentifier.h"

#include <stdlib.h>     // malloc
#include <string.h>     // strdup

struct BamASTIdentifier *BamASTIdentifier_new(char *name) {
    struct BamASTIdentifier *newIdentifier = malloc(sizeof(struct BamASTIdentifier));
    newIdentifier->name = strdup(name);
    return newIdentifier;
}
