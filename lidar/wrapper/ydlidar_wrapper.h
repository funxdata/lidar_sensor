#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void* YDLidarHandle;

/* ⭐ 你自己的跨语言安全结构 */
typedef struct {
    float angle;
    float range;
    float intensity;
} LidarPoint;

YDLidarHandle ydlidar_create();

int ydlidar_initialize(
    YDLidarHandle lidar,
    const char* port,
    int baudrate
);

int ydlidar_turn_on(YDLidarHandle lidar);

int ydlidar_get_scan(
    YDLidarHandle lidar,
    LidarPoint* points,
    int max_points
);

#ifdef __cplusplus
}
#endif
