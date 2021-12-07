mod accelerometer;
mod brake;
mod camera;
mod distance_sensor;
mod distance_sensor_type;
mod gyro;
mod inertial_unit;
mod joint_type;
mod keyboard;
mod motor;
mod position_sensor;
mod robot;
mod robot_mode;
mod touch_sensor;
mod touch_sensor_type;

pub use accelerometer::Accelerometer;
pub use brake::Brake;
pub use camera::Camera;
pub use distance_sensor::DistanceSensor;
pub use distance_sensor_type::DistanceSensorType;
pub use gyro::Gyro;
pub use inertial_unit::InertialUnit;
pub use joint_type::JointType;
pub use keyboard::Keyboard;
pub use motor::Motor;
pub use position_sensor::PositionSensor;
pub use robot::Robot;
pub use robot_mode::RobotMode;
pub use touch_sensor::TouchSensor;
pub use touch_sensor_type::TouchSensorType;

pub use webots_bindings::{
    WbCameraRecognitionObject,
    WB_KEYBOARD_ALT, WB_KEYBOARD_CONTROL, WB_KEYBOARD_DOWN, WB_KEYBOARD_END, WB_KEYBOARD_HOME,
    WB_KEYBOARD_KEY, WB_KEYBOARD_LEFT, WB_KEYBOARD_NUMPAD_DOWN, WB_KEYBOARD_NUMPAD_END,
    WB_KEYBOARD_NUMPAD_HOME, WB_KEYBOARD_NUMPAD_LEFT, WB_KEYBOARD_NUMPAD_RIGHT,
    WB_KEYBOARD_NUMPAD_UP, WB_KEYBOARD_PAGEDOWN, WB_KEYBOARD_PAGEUP, WB_KEYBOARD_RIGHT,
    WB_KEYBOARD_SHIFT, WB_KEYBOARD_UP,
};
