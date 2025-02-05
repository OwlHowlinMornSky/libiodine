#pragma once

class FileMapping {
public:
	FileMapping();
	~FileMapping();

	void* View();

private:
	struct Members;
	Members* m;
};
