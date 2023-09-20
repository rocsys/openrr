use arci::*;
use r2r::{sensor_msgs::msg::LaserScan, QosProfile};
use serde::{Deserialize, Serialize};

use crate::{utils, Node};

/// `arci::LaserScan2D` implementation for ROS2.
pub struct Ros2LaserScan2D {
    node: Node,
    laser_scan_topic_name: String,
}

impl Ros2LaserScan2D {
    /// Creates a new `Ros2LaserScan2D` from sensor_msgs/LaserScan topic name.
    pub fn new(node: Node, laser_scan_topic_name: &str) -> Self {
        Self {
            node,
            laser_scan_topic_name: laser_scan_topic_name.to_owned(),
        }
    }
}

impl LaserScan2D for Ros2LaserScan2D {
    fn current_scan(&self) -> Result<arci::Scan2D, arci::Error> {
        let scan_subscriber = self
            .node
            .r2r()
            .subscribe::<LaserScan>(&self.laser_scan_topic_name, QosProfile::default())
            .unwrap();

        let subscribed_scan =
            utils::spawn_blocking(
                async move { utils::subscribe_one(scan_subscriber).await.unwrap() },
            )
            .join()
            .unwrap();

        let current_scan = match subscribed_scan {
            Some(msg) => Scan2D {
                angle_min: msg.angle_min as f64,
                angle_max: msg.angle_max as f64,
                angle_increment: msg.angle_increment as f64,
                time_increment: msg.time_increment as f64,
                scan_time: msg.scan_time as f64,
                range_min: msg.range_min as f64,
                range_max: msg.range_max as f64,
                ranges: msg.ranges.iter().map(|&v| v as f64).collect::<Vec<f64>>(),
                intensities: msg
                    .intensities
                    .iter()
                    .map(|&v| v as f64)
                    .collect::<Vec<f64>>(),
            },
            None => {
                return Err(Error::Connection {
                    message: format!("Failed to get scan from {}", self.laser_scan_topic_name),
                });
            }
        };

        Ok(current_scan)
    }
}

/// Configuration for `Ros2LaserScan2D`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ros2LaserScan2DConfig {
    /// Topic name for sensor_msgs/LaserScan.
    pub topic: String,
}
