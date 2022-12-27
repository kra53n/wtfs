#pragma once

#include <map>
#include <string>
#include <vector>

#include "file.hpp"

#define PROG_NAME     "wtf"
#define PROG_FULLNAME "What the files"

const std::map<std::string, FileType> files = {
	{ "Python", {
		{ "py", "pyi" },
		"#/bin/python"}
	}, 
	{ "Bash", {
		{ "sh" },
		"#/bin/bash" }
	},

	{ "JavaScrip", {
		{ "js" },
		NONE }
	},

	{ "Cpp", {
		{ "cpp", "hpp", "cc" },
		NONE }
	},

	{ "C", {
		{ "c", "h" },
		NONE }
	},

	{ "Text", {
		{ "txt" },
		NONE }
	},
};
