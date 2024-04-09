#include"Entity.h"

class EntityGroup {
	EntityGroup(int numEntities);
private:
	Entity* mpEntities;
	int mNumEntities;
	float* mpMassValues; 

};

// types of fields
// n-body : gravitational, EM : args charge1, charge2, position1, position2 for each body
// body to body : springs etc. strength,  position1, position2
// scalar : vector 