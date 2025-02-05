#include "FileMapping.h"

#include <iostream>

#define WIN32_LEAN_AND_MEAN
#include <Windows.h>

struct FileMapping::Members {
	HANDLE mapping;
	void* view;
};

FileMapping::FileMapping() {
	m = new Members;

	SECURITY_ATTRIBUTES sa = {};
	sa.nLength = sizeof(sa);
	m->mapping = CreateFileMappingA(
		INVALID_HANDLE_VALUE,
		&sa,
		PAGE_READWRITE | SEC_COMMIT,
		0x0, 0x04000000,
		NULL
	);
	if (m->mapping == NULL) {
		std::cout << "Failed to map " << GetLastError() << '\n';
		return;
	}

	m->view = MapViewOfFile(m->mapping, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, 0);
	if (m->view == NULL) {
		std::cout << "Failed to view\n";
	}
	return;
}

FileMapping::~FileMapping() {
	UnmapViewOfFile(m->view);
	m->view = nullptr;

	CloseHandle(m->mapping);
	m->mapping = NULL;

	delete m;
	m = nullptr;
}

void* FileMapping::View() {
	return m->view;
}
