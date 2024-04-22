#include "Particle_Group.h"
#include<cmath>

struct Spring {
	float distance;
	float k1;
	float k2;
};

struct TwoBodySpring {
	int particle1;
	int particle2;
	Spring spring;
};

const float boxWidth = 1.0;
const float springStrength = 0.5;
const float springDampen = 0.0;
const float smallBoxPos[2] = { 0.0,0.0 };
const float bigBoxPos[2] = { 0.0, -0.5 };


float edgeSpringLength = (2.0 * sqrt(2.0) - 2.0) * boxWidth;
float longEdgeSpringLength = sqrt((393.0 / 64.0) - (15.0 * sqrt(2.0) / 4.0)) * boxWidth;
float midSpringLength = 0.75 * boxWidth;
float edgeToCentreSpringLength = (2.0 - sqrt(2.0)) * boxWidth;
float midToCentreSpringLength = 1.25 * boxWidth;

Spring edgeSpring = { edgeSpringLength,springStrength,springDampen };
Spring longEdgeSpring = { longEdgeSpringLength, springStrength, springDampen };
Spring midSpring = { midSpringLength, springStrength, springDampen };
Spring edgeToCentreSpring = { edgeToCentreSpringLength, springStrength, springDampen };
Spring midToCentreSpring = { midToCentreSpringLength, springStrength, springDampen };



int main() {

	ParticleGroup* particles = new ParticleGroup(13);
	float* masses = new float[13];
	float* sizes = new float[13];

	bool run = true;
    while (run) {

	}
}