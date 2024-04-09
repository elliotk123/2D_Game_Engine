#ifndef CALCULATOR_H
#define CALCULATOR_H

#include "vector_base.h"

class ForceCalculator {
public:
	ForceCalculator(const float* const scalars, const VectorBase** const vectors, void(*mpCalc)(const float*, const VectorBase**, VectorBase*));
	void execute(VectorBase* result);
private:
	const float* mpScalars;
	const VectorBase** mpVectors;
	void(*mpCalc)(const float*, const VectorBase**, VectorBase*);
};
#endif