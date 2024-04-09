// 2D_Physics_Engine.cpp : Defines the entry point for the application.
//

#include <iostream>
#include "Entity.h"
#include "Vector_2d_Euclidean.h"

const int numParticles = 50;

int main()
{
	std::cout << "Hello CMake." << std::endl;

	Entity** entityList = new Entity* [numParticles];
	float* positionsAndVelocities = new float[numParticles*4]; // 50 particles pos_x|pos_y|vel_x|vel_y|

	for (int i = 0; i < numParticles; i++) {
		entityList[i] = new Entity(
			new Vector2dEuclidean(&positionsAndVelocities[i*4]), 
			new Vector2dEuclidean(&positionsAndVelocities[i * 4+2])
		);
	}

	return 0;
}
