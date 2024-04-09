class ConstraintBase {
	virtual float getConstraint() const = 0;
	virtual float getConstraintFirstDerivative() const = 0;
	virtual float getConstraintSecondDerivative() const = 0;
};