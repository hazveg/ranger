use bevy::prelude::*;

/// Returns the angle from the given origin to the given destination.
/// 
/// The reason this is required is because coordinate calculations to attain
/// the angle have the possibility of ending up out of bounds of the window;
/// which leads to the origin facing somewhere that definetely isn't the
/// target.
/// 
/// To right this, this function compares the the origin's values and makes
/// them positive - or negative - so they can be properly calculated.
/// 
/// # Examples
/// 
/// ```
/// use bevy::prelude::*;
/// use ranger::get_angle;
/// 
/// let player = Vec3::new(0.0, 0.0, 0.0);
/// let cursor = Vec3::new(2.0, 2.0, 0.0);
/// 
/// let angle = get_angle(player, cursor);
/// assert_eq!(angle, 0.7853982);
/// 
pub fn get_angle(origin: Vec3, destination: Vec3) -> f32 {
    let x;
    let y;

    if origin.x.is_sign_negative() {
        x = origin.x.abs() + destination.x;
    } else {
        x = -(origin.x) + destination.x;
    }

    if origin.y.is_sign_negative() {
        y = origin.y.abs() + destination.y;
    } else {
        y = -(origin.y) + destination.y;
    }

    y.atan2(x)
}
