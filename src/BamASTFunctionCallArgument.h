#ifndef __BAMASTFUNCTIONCALLARGUMENT_H__
#define __BAMASTFUNCTIONCALLARGUMENT_H__


// c++ std libs
#include <string>

// project
#include "BamASTExpression.h"

class BamASTFunctionCallArgument {
    std::string name;
    BamASTExpression *valueExpression;
public:
    BamASTFunctionCallArgument(std::string name, BamASTExpression *valueExpression) :
        name(name),
        valueExpression(valueExpression)
        {}
    void print();
};


#endif  // __BAMASTFUNCTIONCALLARGUMENT_H__
