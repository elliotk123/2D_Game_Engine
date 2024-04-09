# include "force_accumulator.h"

ForceAccumulator::ForceAccumulator(VectorBase* const forceVector) {
	mpForce = forceVector;
}

void ForceAccumulator::add(const VectorBase* forceVector) {
	mpForce->Add(forceVector);
}

void ForceAccumulator::reset(const VectorBase* const forceVector) {
	*mpForce = *forceVector;
}

VectorBase* ForceAccumulator::getForce() const {
	return mpForce;
}
