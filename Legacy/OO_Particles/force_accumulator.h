#ifndef FORCEACCUMULATOR_H
#define FORCEACCUMULATOR_H
#include "vector_base.h"

class ForceAccumulator {
public: 
	ForceAccumulator(VectorBase* const forceVector);
	void add(const VectorBase* force);
	void reset(const VectorBase* const forceVector);
	VectorBase* getForce() const;
private:
	VectorBase* mpForce;
    int num_calcs;
};

#endif