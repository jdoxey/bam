#ifndef __BAMASTLITERALSTRING_H__
#define __BAMASTLITERALSTRING_H__


// c++ std libs
#include <string>

// project
#include "BamASTExpression.h"

class BamASTLiteralString : public BamASTExpression {
	std::string contents;
public:
	BamASTLiteralString(std::string contents)
		: contents(contents)
		{};
	void print();
};


#endif  // __BAMASTLITERALSTRING_H__
