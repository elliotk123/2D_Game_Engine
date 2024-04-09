#include "Entity.h"

Entity::Entity(VectorBase* posn, VectorBase* velocity) {
	Entity::Posn = posn;
	Entity::Velocity = velocity;
}

Entity::~Entity() {
	delete Entity::Posn;
	delete Entity::Velocity;

}

void Entity::Set_Position(VectorBase* new_posn) {
	*Entity::Posn = *new_posn;
};

void Entity::Set_Velocity(VectorBase* new_velocity) {
	*Entity::Velocity = *new_velocity;
}
VectorBase* Entity::Get_Position() const{
	return Posn;
}
VectorBase* Entity::Get_Velocity() const{
	return Velocity;
}