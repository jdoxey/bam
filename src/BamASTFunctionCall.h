#ifndef __BAMASTFUNCTIONCALL_H__
#define __BAMASTFUNCTIONCALL_H__


// c++ std libs
#include <string>

// project
#include "BamASTExpression.h"
#include "BamASTFunctionCallArgument.h"

class BamASTFunctionCall : public BamASTExpression {
	std::string functionName;
	BamASTFunctionCallArgument *argument;
public:
	BamASTFunctionCall(std::string functionName, BamASTFunctionCallArgument *argument) :
		functionName(functionName),
		argument(argument)
		{}
	void print();
};


#endif  // __BAMASTFUNCTIONCALL_H__
