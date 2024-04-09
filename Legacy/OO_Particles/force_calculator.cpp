#include "force_calculator.h"

ForceCalculator::ForceCalculator(const float* const scalars, const VectorBase** const vectors, void(*calc)(const float*, const VectorBase**, VectorBase*)) {
	ForceCalculator::mpScalars = scalars;
	ForceCalculator::mpVectors = vectors;
	ForceCalculator::mpCalc = calc;
}

void ForceCalculator::execute(VectorBase* const result) {
	ForceCalculator::mpCalc(ForceCalculator::mpScalars, ForceCalculator::mpVectors, result);
}