use bevy::prelude::*;
use physx::prelude::*;

pub trait IntoPxVec3 {
    fn to_physx(&self) -> PxVec3;
    fn to_physx_sys(&self) -> physx_sys::PxVec3;
}

pub trait IntoBevyVec3 {
    fn to_bevy(&self) -> Vec3;
}

pub trait IntoPxQuat {
    fn to_physx(&self) -> PxQuat;
}

pub trait IntoBevyQuat {
    fn to_bevy(&self) -> Quat;
}

pub trait IntoPxTransform {
    fn to_physx(&self) -> PxTransform;
}

pub trait IntoBevyTransform {
    fn to_bevy(&self) -> Transform;
}

impl IntoPxVec3 for Vec3 {
    fn to_physx(&self) -> PxVec3 {
        PxVec3::new(self.x, self.y, self.z)
    }

    fn to_physx_sys(&self) -> physx_sys::PxVec3 {
        physx_sys::PxVec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl IntoBevyVec3 for PxVec3 {
    fn to_bevy(&self) -> Vec3 {
        Vec3::new(self.x(), self.y(), self.z())
    }
}

impl IntoBevyVec3 for physx_sys::PxVec3 {
    fn to_bevy(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoPxQuat for Quat {
    fn to_physx(&self) -> PxQuat {
        PxQuat::new(self.x, self.y, self.z, self.w)
    }
}

impl IntoBevyQuat for PxQuat {
    fn to_bevy(&self) -> Quat {
        Quat::from_xyzw(self.x(), self.y(), self.z(), self.w())
    }
}

impl IntoBevyQuat for physx_sys::PxQuat {
    fn to_bevy(&self) -> Quat {
        Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }
}

impl IntoPxTransform for Transform {
    fn to_physx(&self) -> PxTransform {
        PxTransform::from_translation_rotation(
            &self.translation.to_physx(),
            &self.rotation.to_physx(),
        )
    }
}

impl IntoPxTransform for GlobalTransform {
    fn to_physx(&self) -> PxTransform {
        let (_scale, rotation, translation) = self.to_scale_rotation_translation();

        PxTransform::from_translation_rotation(
            &translation.to_physx(),
            &rotation.to_physx(),
        )
    }
}

impl IntoBevyTransform for PxTransform {
    fn to_bevy(&self) -> Transform {
        Transform {
            translation: self.translation().to_bevy(),
            rotation: self.rotation().to_bevy(),
            scale: Vec3::splat(1.),
        }
    }
}

impl IntoBevyTransform for physx_sys::PxTransform {
    fn to_bevy(&self) -> Transform {
        Transform {
            translation: self.p.to_bevy(),
            rotation: self.q.to_bevy(),
            scale: Vec3::splat(1.),
        }
    }
}
