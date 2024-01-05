use bevy::math::IVec2;

trait IVec2Ext {
    fn manhattan(&self, other: IVec2) -> i32;
}

impl IVec2Ext for IVec2 {
    fn manhattan(&self, other: IVec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
