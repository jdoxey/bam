%{
#include "BamParser.h"
using namespace yy;
%}


%option noyywrap outfile="build-files/BamScanner.cpp" header="build-files/BamScanner.h"
%option reentrant interactive noyylineno

%%

":"                     return BamParser::token::COLON;
"("                     return BamParser::token::OPEN_BRACKET;
")"                     return BamParser::token::CLOSE_BRACKET;
[a-zA-Z_][a-zA-Z_0-9]*  yylval->build<std::string>(yytext); return BamParser::token::IDENTIFIER;
["][^"]*["]             yylval->build<std::string>(yytext); return BamParser::token::LITERAL_STRING;
.                       /* ignore anything we don't recognise */

%%
