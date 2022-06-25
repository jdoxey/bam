%{
#include "BamScanner.h"
%}

// To appear in generated header
%code requires {
  #include "BamASTStatementList.h"
  #include "BamASTStatement.h"
  #include "BamASTExpression.h"
  #include "BamASTFunctionCall.h"
  #include "BamASTFunctionCallArgument.h"
  #include "BamASTLiteralString.h"
}

%language "c++"
%require "3.2"
%defines "build-files/BamParser.h"
%output "build-files/BamParser.cpp"

%define api.parser.class {BamParser}
%define api.value.type variant
%param {yyscan_t scanner}
 
%code provides
{
    #define YY_DECL \
        int yylex(yy::BamParser::semantic_type *yylval, yyscan_t yyscanner)
    YY_DECL;
}

%type <BamASTStatementList *> statementList;
%type <BamASTStatement *> statement;
%type <BamASTLiteralString *> stringLiteral;
%type <BamASTFunctionCall *> functionCall;
%type <BamASTFunctionCallArgument *> functionCallArgument;
%type <BamASTExpression *> expression;

%token <std::string> IDENTIFIER
%token <std::string> LITERAL_STRING
%token OPEN_BRACKET CLOSE_BRACKET COLON

%%

program:
  statementList {
    $1->print();
  }
;

statementList:
  %empty {}
| statementList statement {
    $$ = new BamASTStatementList($1, $2);
  }
;

// 'statement's which are not 'expression's are things that don't make
// sense in an 'if' condition.
statement:
  expression {
    $$ = (BamASTStatement *)$1;
  }
;

// expression is a small code snippet which can be used in many places,
// like the value for an argument, the condition inside an 'if', etc...
expression:
  functionCall {
    $$ = (BamASTExpression *)$1;
  }
| stringLiteral {
    $$ = (BamASTExpression *)$1;
  } 
;

// Literals
stringLiteral:
  LITERAL_STRING {
    $$ = new BamASTLiteralString($1);
  }
;

// Function Call

// TODO: all function calls (temporarily) have exactly one argument
functionCall:
  IDENTIFIER OPEN_BRACKET functionCallArgument CLOSE_BRACKET {
    $$ = new BamASTFunctionCall($1, $3);
  }
;

functionCallArgument:
  IDENTIFIER COLON expression {
    $$ = new BamASTFunctionCallArgument($1, $3);
  }
;


%%


void yy::BamParser::error (const std::string& m) {
  std::cerr << m << '\n';
}

int main(int argc, char *argv[]) {
    yyscan_t scanner;
    yylex_init(&scanner);
    yy::BamParser parser{ scanner };
    std::cout.precision(10);
    parser.parse();
    yylex_destroy(scanner);
}
