use anyhow::bail;
use webots_bindings::{
    wb_device_get_node_type, wb_inertial_unit_disable, wb_inertial_unit_enable,
    wb_inertial_unit_get_noise, wb_inertial_unit_get_quaternion,
    wb_inertial_unit_get_roll_pitch_yaw, wb_inertial_unit_get_sampling_period, WbDeviceTag,
    WbNodeType_WB_NODE_INERTIAL_UNIT,
};

pub struct InertialUnit(WbDeviceTag);

impl InertialUnit {
    pub(crate) fn new(device: WbDeviceTag) -> Self {
        assert_eq!(WbNodeType_WB_NODE_INERTIAL_UNIT, unsafe {
            wb_device_get_node_type(device)
        });
        Self(device)
    }

    pub fn enable(&self, sampling_period: i32) {
        unsafe { wb_inertial_unit_enable(self.0, sampling_period) }
    }

    pub fn disable(&self) {
        unsafe { wb_inertial_unit_disable(self.0) }
    }

    pub fn get_sampling_period(&self) -> i32 {
        unsafe { wb_inertial_unit_get_sampling_period(self.0) }
    }

    pub fn get_noise(&self) -> f64 {
        unsafe { wb_inertial_unit_get_noise(self.0) }
    }

    pub fn get_roll_pitch_yaw(&self) -> anyhow::Result<[f64; 3]> {
        unsafe {
            let roll_pitch_yaw = wb_inertial_unit_get_roll_pitch_yaw(self.0);
            if roll_pitch_yaw.is_null() {
                bail!("Failed to get roll/pitch/yaw: roll/pitch/yaw data is NULL");
            }
            Ok([
                *roll_pitch_yaw.offset(0),
                *roll_pitch_yaw.offset(1),
                *roll_pitch_yaw.offset(2),
            ])
        }
    }

    pub fn get_quaternion(&self) -> anyhow::Result<[f64; 4]> {
        unsafe {
            let quaternion = wb_inertial_unit_get_quaternion(self.0);
            if quaternion.is_null() {
                bail!("Failed to get quaternion: quaternion data is NULL");
            }
            Ok([
                *quaternion.offset(0),
                *quaternion.offset(1),
                *quaternion.offset(2),
                *quaternion.offset(3),
            ])
        }
    }
}
