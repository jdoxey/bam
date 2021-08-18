%{
#include <stdio.h>
%}

%token IDENTIFIER
%token SEMICOLON
%token NEWLINE

%%

list-of-statements: /* empty */
                  | list-of-statements statement
                  | list-of-statements statement end-of-statement
				  ;

statement: IDENTIFIER     { puts("IDENTIFIER"); };

end-of-statement: SEMICOLON | NEWLINE ;

%%

int main(int argc, char **argv) {
	yyparse();
}

int yyerror(const char *s) {
	fprintf(stderr, "error: %s\n", s);
}
