#include "Vector_2d_Euclidean.h"
#include <math.h>

Vector2dEuclidean::Vector2dEuclidean(float* elements) : VectorBase(elements) {}

float Vector2dEuclidean::Calc_Length() const{
	return sqrt(Elements[0] * Elements[0] + Elements[1] * Elements[1]);
}

float Vector2dEuclidean::Dot_Prod(const VectorBase* vector) const {
	return Elements[0] * vector->Get_Element(0) + Elements[1] * vector->Get_Element(1);
}

void Vector2dEuclidean::Scale(float scale, VectorBase* result) const {
	result->Set_Element(0, scale * Elements[0]);
	result->Set_Element(1, scale * Elements[1]);
}

void Vector2dEuclidean::Add(const VectorBase* vector, VectorBase* result) const{
	Vector2dEuclidean* Result = new Vector2dEuclidean(new float[2]);
	result->Set_Element(0, Elements[0] + vector->Get_Element(0));
	result->Set_Element(1, Elements[1] + vector->Get_Element(1));
}

void Vector2dEuclidean::Add(const VectorBase* vector) {
	Elements[0] += vector->Get_Element(0);
	Elements[1] += vector->Get_Element(1);
}

void Vector2dEuclidean::Subtract(const VectorBase* vector, VectorBase* result) const{
	Vector2dEuclidean* Result = new Vector2dEuclidean(new float[2]);
	result->Set_Element(0, Elements[0] - vector->Get_Element(0));
	result->Set_Element(1, Elements[1] - vector->Get_Element(1));
}