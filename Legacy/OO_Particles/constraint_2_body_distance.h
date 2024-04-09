#include<constraint_base.h>
#include<entity.h>

class Constraint2BodyDistance public ConstraintBase{
public:
	override float getConstraint() const;
private:
		Entity* mpEntity1;
		Entity* mpEntity2;
		float distance;
};