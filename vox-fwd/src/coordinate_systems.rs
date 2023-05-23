use crate::*;

use crate::safety::TaggedAs;
use std::marker::PhantomData;

pub struct Mesh;
pub struct Model;
pub struct World;
pub struct View;
pub struct Clip;
pub struct Screen;

pub struct In<CS> {
    _phantoms: PhantomData<CS>,
}

impl<CS> In<CS> {
    fn from<T>(t: T) -> TaggedAs<In<CS>, T> {
        t.into()
    }
}

pub type InClip<T>  = TaggedAs<In<Clip>, T>;
pub type InMesh<T>  = TaggedAs<In<Mesh>, T>;
pub type InModel<T> = TaggedAs<In<Model>, T>;
pub type InView<T>  = TaggedAs<In<View>, T>;
pub type InWorld<T> = TaggedAs<In<World>, T>;

pub type PtMesh    = InModel<Pt3>;
pub type PtModel   = TaggedAs<In<Model>, Pt3>;
pub type PtWorld   = TaggedAs<In<World>, Pt3>;
pub type PtView    = TaggedAs<In<View>, Pt3>;
pub type PtClip    = TaggedAs<In<Clip>, Pt3>;

pub type VecMesh    = TaggedAs<In<Mesh>, Vec3>;
pub type VecModel   = TaggedAs<In<Model>, Vec3>;
pub type VecWorld   = TaggedAs<In<World>, Vec3>;
pub type VecView    = TaggedAs<In<View>, Vec3>;
pub type VecClip    = TaggedAs<In<Clip>, Vec3>;

pub type PxScreen  = TaggedAs<In<Screen>, Px2>;

pub mod transforms {
    use std::marker::PhantomData;
    use std::ops::{Add, AddAssign, Mul, Sub};

    use super::*;
    use crate::safety::TaggedAs;

    pub struct Tx<Dst, Src> {
        _d: PhantomData<(Dst,Src)>,
    }

    pub type WorldFromModel<Xform = na::Affine3<f32>>  = TaggedAs<Tx<World, Model>, Xform, >;
    pub type ModelFromWorld<Xform = na::Affine3<f32>>  = TaggedAs<Tx<Model, World>, Xform>;
    pub type ModelFromMesh<Xform = na::Affine3<f32>>   = TaggedAs<Tx<Model, Mesh>, Xform>;
    pub type MeshFromModel<Xform = na::Affine3<f32>>   = TaggedAs<Tx<Mesh, Model>, Xform>;

    // todo: complete this lineup...
    pub type WorldFromWorld<Xform = na::Affine3<f32>>  = TaggedAs<Tx<World, World>, Xform>;
    pub type ModelFromModel<Xform = na::Affine3<f32>>  = TaggedAs<Tx<Model, Model>, Xform>;
    pub type MeshFromMesh<Xform = na::Affine3<f32>>    = TaggedAs<Tx<Mesh, Mesh>, Xform>;

    // todo: document this...
    impl<Dst, Src, T, P> Mul<TaggedAs<In<Src>, T>> for TaggedAs<Tx<Dst, Src>, P>
        where P: Mul<T>
    {
        type Output = TaggedAs<In<Dst>, <P as Mul<T>>::Output>;

        fn mul(self, rhs: TaggedAs<In<Src>, T>) -> Self::Output {
            (self.value * rhs.value).into()
        }
    }

    // todo: document this...
    impl<A, B, C, Lhs, Rhs> Mul<TaggedAs<Tx<B, C>, Rhs>> for TaggedAs<Tx<A, B>, Lhs>
        where Lhs: Mul<Rhs>
    {
        type Output = TaggedAs<Tx<A, C>, <Lhs as Mul<Rhs>>::Output>;

        fn mul(self, rhs: TaggedAs<Tx<B, C>, Rhs>) -> Self::Output {
            (self.value * rhs.value).into()
        }
    }

    // todo: document this...
    macro_rules! define_add_assign {
        ($Lhs:ty, $Rhs:ty) => {
            impl<CS> AddAssign<TaggedAs<In<CS>, $Rhs>> for TaggedAs<In<CS>, $Lhs>
                where $Lhs: AddAssign<$Rhs>
            {
                fn add_assign(&mut self, rhs: TaggedAs<In<CS>, $Rhs>) {
                    self.value += rhs.value
                }
            }
        };
    }

    // todo: document this...
    macro_rules! define_add {
        ( $Lhs:ty, $Rhs:ty ) => {
            impl<CS> Add<TaggedAs<In<CS>, $Rhs>> for TaggedAs<In<CS>, $Lhs>
                where $Lhs: Add<$Rhs>
            {
                type Output = TaggedAs<In<CS>, <$Lhs as Add<$Rhs>>::Output>;

                fn add(self, rhs: TaggedAs<In<CS>, $Rhs>) -> Self::Output {
                    (self.value + rhs.value).into()
                }
            }
        };
    }

    // macro_rules! define_sub {
    //     ( $Lhs:ty, $Rhs:ty ) => {
    //         impl<CS> Sub<TaggedAs<$Rhs, In<CS>>> for TaggedAs<$Lhs, In<CS>>
    //             where $Lhs: Sub<$Rhs>
    //         {
    //             type Output = TaggedAs<<$Lhs as Sub<$Rhs>>::Output, In<CS>>;
    //
    //             fn sub(&mut self, rhs: TaggedAs<$Rhs, In<CS>>) -> Self::Output {
    //                 (self.value - rhs.value).into()
    //             }
    //         }
    //     };
    // }

    // todo: document this...
    macro_rules! define_mul {
        ( $Lhs:ty, $Rhs:ty ) => {
            impl<CS> Mul<TaggedAs<In<CS>, $Rhs>> for TaggedAs<In<CS>, $Lhs>
                where $Lhs: Mul<$Rhs>
            {
                type Output = TaggedAs<In<CS>, <$Lhs as Mul<$Rhs>>::Output>;

                fn mul(self, rhs: TaggedAs<In<CS>, $Rhs>) -> Self::Output {
                    (self.value * rhs.value).into()
                }
            }
        };
    }

    // todo: document this...
    macro_rules! define_mul_assign {
        ( $Lhs:ty, $Rhs:ty ) => {
            impl<CS> std::ops::MulAssign<TaggedAs<In<CS>, $Rhs>> for TaggedAs<In<CS>, $Lhs>
                where $Lhs: std::ops::MulAssign<$Rhs>
            {
                fn mul_assign(&mut self, rhs: TaggedAs<In<CS>, $Rhs>) {
                    self.value *= rhs.value;
                }
            }
        };
    }

    // todo: document this...
    macro_rules! define_non_trait_xforms_identity {
        ($T:ty) => {
            impl<CS> TaggedAs<In<CS>, $T> {
                pub fn identity() -> Self {
                    Self::from(<$T>::identity())
                }

                pub fn transform_point(&self, p: &TaggedAs<In<CS>, Pt3>) -> TaggedAs<In<CS>, Pt3> {
                    (self.value.transform_point(&p.value)).into()
                }
            }
        };
    }

    // todo: document this...
    macro_rules! define_transform_point {
       ($Src:tt, $Dst:ty) => {

       }
    }

    // todo: document this...
    macro_rules! define_non_trait_xforms {
        (1, $T:ty) => {
            impl<Dst, Src> TaggedAs<Tx<Dst, Src>, $T> {
                pub fn identity() -> Self {
                    Self::from(<$T>::identity())
                }

                pub fn transform_point(&self, p: &TaggedAs<In<Src>, Pt3>) -> TaggedAs<In<Dst>, Pt3> {
                    (self.value.transform_point(&p.value)).into()
                }

                pub fn transform_vector(&self, p: &TaggedAs<In<Src>, Vec3>) -> TaggedAs<In<Dst>, Vec3> {
                    (self.value.transform_vector(&p.value)).into()
                }
            }
        };

        ($T:ty) => {
            impl<Dst, Src> TaggedAs<Tx<Dst, Src>, $T> {
                pub fn identity() -> Self {
                    Self::from(<$T>::identity())
                }

                pub fn transform_point(&self, p: &TaggedAs<In<Src>, Pt3>) -> TaggedAs<In<Dst>, Pt3> {
                    (self.value.transform_point(&p.value)).into()
                }
            }
        };
    }

    define_add!(Pt3, Vec3);
    define_add!(Vec3, Vec3);
    define_add_assign!(Pt3, Vec3);
    define_add_assign!(Vec3, Vec3);

    define_add!(na::Quaternion<f32>, na::Quaternion<f32>);

    define_mul!(Pt3, f32);
    define_mul!(Vec3, f32);
    define_mul_assign!(Pt3, f32);
    define_mul_assign!(Vec3, f32);

    define_mul!(na::Translation3<f32>, Pt3);
    define_mul!(na::Translation3<f32>, na::Rotation3<f32>);
    define_mul!(na::Translation3<f32>, na::UnitQuaternion<f32>);

    define_mul!(na::Rotation3<f32>, Pt3);
    define_mul!(na::Rotation3<f32>, Vec3);
    define_mul!(na::Rotation3<f32>, na::Translation3<f32>);
    define_mul!(na::Rotation3<f32>, na::UnitQuaternion<f32>);

    define_mul!(na::Quaternion<f32>, na::Quaternion<f32>);

    // --
    // These definitions are to get things like 'transform_point(...)' and '::identity()'.
    define_non_trait_xforms!(1, na::Transform3<f32>);
    define_non_trait_xforms!(na::Translation3<f32>);
    define_non_trait_xforms!(1, na::Rotation3<f32>);
    define_non_trait_xforms!(1, na::Isometry3<f32>);
    define_non_trait_xforms!(1, na::Similarity3<f32>);
    define_non_trait_xforms!(1, na::Projective3<f32>);
    define_non_trait_xforms!(1, na::Affine3<f32>);

    // projective and generalized transforms don't have am 'identity' shorthand because
    // they are normally used to convert from one coordinate System than another.
    define_non_trait_xforms_identity!(na::Translation3<f32>);
    define_non_trait_xforms_identity!(na::Rotation3<f32>);
    define_non_trait_xforms_identity!(na::Isometry3<f32>);
    define_non_trait_xforms_identity!(na::Similarity3<f32>);
    define_non_trait_xforms_identity!(na::Affine3<f32>);
}
