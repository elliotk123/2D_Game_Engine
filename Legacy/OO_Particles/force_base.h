#ifndef FORCEBASE_H
#define FORCEBASE_H

#include "entity_group.h"
#include "force_accumulator.h"

class ForceBase {
public:
	virtual void calculateForces(EntityGroup* entityGroup, ForceAccumulator* forceAccumulator) = 0;
private:

};

#endif