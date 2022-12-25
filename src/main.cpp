#include <fstream>
#include <iostream>
#include <filesystem>

#include "config.hpp"

namespace fs = std::filesystem;

int main(int argc, char** argv) {
	fs::path path(fs::current_path());
	std::cout << std::endl << path << std::endl;
	return 0;
}
