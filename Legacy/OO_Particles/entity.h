#include "Vector_Base.h"
class Entity{
private:
	VectorBase* mpPosn;
	VectorBase* mpVelocity;
public:
	Entity(VectorBase* posn, VectorBase* velocity);
	~Entity();
	void Set_Position(VectorBase* new_posn);
	void Set_Velocity(VectorBase* new_velocity);
	VectorBase* Get_Position() const;
	VectorBase* Get_Velocity() const;
};