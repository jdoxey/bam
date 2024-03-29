%option noyywrap

%{
#include "bamBisonParser.tab.h"
%}

%%

";"                        { return SEMICOLON; }
[a-zA-Z_][a-zA-Z0-9_]*     { yylval.string = yytext; return IDENTIFIER; }
\r?\n                      { return NEWLINE; }
[ \t]                      { /* ignore whitespace */ }

%%
