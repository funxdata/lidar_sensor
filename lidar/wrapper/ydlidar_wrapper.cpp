#include "ydlidar_wrapper.h"

#include "CYdLidar.h"
#include "ydlidar_def.h"

using namespace ydlidar;

struct LidarWrapper {
    CYdLidar laser;
};

YDLidarHandle ydlidar_create() {
    return new LidarWrapper();
}

int ydlidar_initialize(
    YDLidarHandle handle,
    const char* port,
    int baudrate
) {
    auto* wrapper = (LidarWrapper*)handle;

    wrapper->laser.setlidaropt(
        LidarPropSerialPort,
        (void*)port,
        strlen(port)
    );

    wrapper->laser.setlidaropt(
        LidarPropSerialBaudrate,
        (void*)&baudrate,
        sizeof(int)
    );

    int type = TYPE_TRIANGLE;

    wrapper->laser.setlidaropt(
        LidarPropLidarType,
        &type,
        sizeof(int)
    );

    return wrapper->laser.initialize();
}

int ydlidar_turn_on(
    YDLidarHandle handle
) {
    auto* wrapper = (LidarWrapper*)handle;
    return wrapper->laser.turnOn();
}


int ydlidar_get_scan(
    YDLidarHandle handle,
    LidarPoint* out_points,
    int max_points
) {
    auto* wrapper = (LidarWrapper*)handle;

    LaserScan scan;

    if (!wrapper->laser.doProcessSimple(scan)) {
        return 0;
    }

    int n = std::min((int)scan.points.size(), max_points);

    for (int i = 0; i < n; i++) {
        out_points[i].angle = scan.points[i].angle;
        out_points[i].range = scan.points[i].range;
        out_points[i].intensity = scan.points[i].intensity;
    }

    return n;
}
~