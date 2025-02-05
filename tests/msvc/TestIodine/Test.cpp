
#include "Test.h"

#include <iostream>
#include <opencv2/opencv.hpp>

#include <libiodine.h>
#include "FileMapping.h"

using namespace std;

void TestFileMapping() {
	char input[] = "test.jpg";

	FileMapping map;

	void* view = map.View();
	if (view == nullptr)
		return;

	CSI_Parameters parameters = {};
	parameters.keep_metadata = false;
	parameters.jpeg_quality = 60;
	parameters.png_quality = 1;

	CSI_Result res = csi_convert_into(input, view, 0x04000000, CSI_SupportedFileTypes::Jpeg, &parameters);
	if (res.success) {
		cout << "Succeed: " << res.code << "." << endl;
	}
	else {
		cout << "Failed: " << res.error_message << " (" << res.code << ")." << endl;
		return;
	}

	cv::_InputArray pic_arr((char*)view, res.code);
	cv::Mat src_mat = cv::imdecode(pic_arr, cv::IMREAD_UNCHANGED);
	try {
		cv::imshow("show", src_mat);
	}
	catch (cv::Exception& e) {
		cout << e.what() << '\n';
	}
	cv::waitKey(0);

	return;
}

void TestResize() {
	char input[] = "36.jpg";
	char output[] = "out2.jpg";

	CSI_Parameters parameters = {};
	parameters.keep_metadata = false;
	parameters.jpeg_quality = 80;
	parameters.png_quality = 1;
	parameters.allow_magnify = false;
	parameters.short_side_pixels = 2400;
	//parameters.reduce_by_power_of_2 = true;

	//CSI_Result res = csi_convert(input, output, SupportedFileTypes::Png, &parameters);
	CSI_Result res = csi_compress(input, output, &parameters);
	if (res.success) {
		cout << "Succeed: " << res.code << "." << endl;
	}
	else {
		cout << "Failed: " << res.error_message << " (" << res.code << ")." << endl;
		return;
	}

	return;
}
