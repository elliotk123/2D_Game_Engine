#include "Vector_Base.h"

class Vector2dEuclidean : public VectorBase {
public:
	Vector2dEuclidean(float* elements);
	float Calc_Length() const override;
	float Dot_Prod(const VectorBase* vector) const override;
	void Scale(float scale, VectorBase* result) const override;
	void Add(const VectorBase* vector, VectorBase* result) const override;
	void Add(const VectorBase* vector) override;
	void Subtract(const VectorBase* vector, VectorBase* result) const override;

};