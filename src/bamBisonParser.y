%{
#include <stdio.h>
#include "BamASTStatement.h"
#include "BamASTListOfStatements.h"
#include "BamASTIdentifier.h"
%}

%union {
    struct BamASTStatement *statement;
    struct BamASTListOfStatements *listOfStatements;
    struct BamASTIdentifier *identifier;
    char *string;
    double number;
}

%token <string> IDENTIFIER
%token SEMICOLON
%token NEWLINE

%type <statement> statement
%type <listOfStatements> program listOfStatements

%%

program          : listOfStatements statement                     { $$ = BamASTListOfStatements_new($1, $2); BamASTListOfStatements_print($$); }
                 | listOfStatements statement endOfStatement      { $$ = BamASTListOfStatements_new($1, $2); BamASTListOfStatements_print($$); }
                 | listOfStatements statement SEMICOLON NEWLINE   { $$ = BamASTListOfStatements_new($1, $2); BamASTListOfStatements_print($$); }
                 ;

listOfStatements : /* empty */                                    { $$ = NULL; }
                 | listOfStatements statement endOfStatement      { $$ = BamASTListOfStatements_new($1, $2); }
                 ;

statement        : IDENTIFIER     { $$ = BamASTStatement_newIdentifier(BamASTIdentifier_new($1)); }
                 ;

endOfStatement   : SEMICOLON
                 | NEWLINE
                 ;

%%

int main(int argc, char **argv) {
	return yyparse();
}

int yyerror(const char *s) {
	fprintf(stderr, "error: %s\n", s);
    return 0;
}
