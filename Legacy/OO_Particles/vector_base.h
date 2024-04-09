#ifndef VECTORBASE_H
#define VECTORBASE_H

class VectorBase {
protected:
	float* Elements;
public:
	VectorBase(float* elements);
	~VectorBase();
	virtual float Calc_Length() const = 0;
	virtual float Dot_Prod(const VectorBase* vector) const = 0;
	virtual void Add(const VectorBase* vector, VectorBase* result) const = 0;
	virtual void Add(const VectorBase* vector) = 0;
	virtual void Subtract(const VectorBase* vector, VectorBase* result)const  = 0;
	virtual void Scale(float scale, VectorBase* result) const = 0;
	float Get_Element(int element) const;
	void Set_Element(int element, float value);
};
#endif // !VECTORBASE_H
