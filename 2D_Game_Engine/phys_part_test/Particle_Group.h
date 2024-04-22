class ParticleGroup {
private:
	float* mpData;
	int mNumParticles;
public:
	ParticleGroup(int numParticles);
	float* GetPosition(int particle);
	void setPosition(float* position, int particle);
	void setPositions(float* positions);
	float* GetVelocity(int particle);
	void setVelocity(float* velocity, int particle);
	float* GetAcceleration(int particle);
	void setAcceleration(float* acceleration, int particle);
	void Iterate(float delta);
	~ParticleGroup();
};