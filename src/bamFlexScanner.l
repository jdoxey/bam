%option noyywrap

%{
#include <stdio.h>
%}

%%

[a-zA-Z_][a-zA-Z0-9_]*     { printf("IDENTIFIER: %s\n", yytext); }
\r?\n                      { printf("NEWLINE\n"); }
[ \t]                      { }

%%

int main(int argc, char **argv) {
	yylex();
}
