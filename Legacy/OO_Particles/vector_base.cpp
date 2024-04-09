#include "Vector_Base.h"

VectorBase::VectorBase(float* elements) {
	VectorBase::Elements = elements;
}
VectorBase::~VectorBase() {
	delete[] Elements;
}
float VectorBase::Get_Element(int element) const{
	return VectorBase::Elements[element];
}

void VectorBase::Set_Element(int element, float value) {
	VectorBase::Elements[element] = value;
}
