%{
#include <stdio.h>
%}

%token IDENTIFIER
%token SEMICOLON
%token NEWLINE

%%

program          : listOfStatements statement
                 | listOfStatements statement endOfStatement
                 | listOfStatements statement SEMICOLON NEWLINE
                 ;

listOfStatements : /* empty */
                 | listOfStatements statement endOfStatement
                 ;

statement        : IDENTIFIER     { puts("IDENTIFIER"); };

endOfStatement   : SEMICOLON | NEWLINE ;

%%

int main(int argc, char **argv) {
	yyparse();
}

int yyerror(const char *s) {
	fprintf(stderr, "error: %s\n", s);
}
