#include"Particle_Group.h"
#include<iostream>

ParticleGroup::ParticleGroup(int numParticles) {
	mNumParticles = numParticles;
	mpData = new float[numParticles*6];
}

ParticleGroup::~ParticleGroup() {
	delete[] mpData;
}

float* ParticleGroup::GetPosition(int particle) {
	float* position;
	if (particle <= mNumParticles && particle > 0) {
		position = mpData + ((particle - 1) * 6);
	}
	else {
		position = mpData;
		std::cout << "particle " << particle << " out of bounds." << std::endl;
	}
	return position;
}

void ParticleGroup::setPosition(float* position, int particle) {
	if (particle <= mNumParticles && particle > 0) {
		mpData[(particle - 1) * 6] = position[0];
		mpData[(particle - 1) * 6 + 1] = position[1];
	}
	else {
		std::cout << "particle " << particle << " out of bounds." << std::endl;
	}
}

void ParticleGroup::setPositions(float* positions) {
	for (int i = 0; i < mNumParticles; i++) {
		mpData[i * 6] = positions[i];
		mpData[i * 6 + 1] = positions[i + 1];
	}
}

float* ParticleGroup::GetVelocity(int particle) {
	float* velocity;
	if (particle <= mNumParticles && particle > 0) {
		velocity = mpData+((particle - 1) * 6 + 2);
	}
	else {
		velocity = mpData;
		std::cout << "particle " << particle << " out of bounds." << std::endl;
	}
	return velocity;
}

float* ParticleGroup::GetAcceleration(int particle) {
	float* acceleration;
	if (particle <= mNumParticles && particle > 0) {
		acceleration = mpData + ((particle - 1) * 6 + 4);
	}
	else {
		acceleration = mpData;
		std::cout << "particle " << particle << " out of bounds." << std::endl;
	}
	return acceleration;
}

void ParticleGroup::Iterate(float delta) {
	float delta_t_squared_over_2 = (delta * delta) / 2;
	for (int i = 0; i < mNumParticles; i= i + 6) {
		mpData[i] = mpData[i] + mpData[i + 2] * delta + mpData[i + 4] * delta_t_squared_over_2;
		mpData[i+1] = mpData[i+1] + mpData[i + 3] * delta + mpData[i + 5] * delta_t_squared_over_2;
		mpData[i + 2] = mpData[i + 2] + mpData[i + 4] * delta;
		mpData[i + 3] = mpData[i + 3] = mpData[i + 5] * delta;
	}
}