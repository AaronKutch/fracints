#![feature(proc_macro_hygiene)]

use normints::*;
use normints_macros::*;
use specialized_div_rem::i128_div_rem;
use std::error::Error;
use std::fmt::Display;
use std::convert::TryFrom;

// Use front-to-back rendering and a depth buffer for the best transparency results
// Check if large swaths have no transparency left

#[derive(Debug)]
pub enum CamError {
    BufferOverflow,
    NegativeDimensions,
    ProjectionPlaneOverflow,
    DegenerateProjection,
}

impl Display for CamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
        /*match self {
            BufferOverflow => write!(f, "Inputs caused buffer sizes to overflow `usize`"),
            NegativeDimensions => write!(f, "Some inputs that should be positive are negative"),
            ProjectionPlaneOverflow => write!(f, "The projection plane size plus the origin overflows"),
            DegenerateProjection => write!(f, "The `focal_len` is 0")
        }*/
    }
}

impl Error for CamError {}

type Vec2 = (fi64, fi64);
type Vec3 = (fi64, fi64, fi64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cam {
    /// Position of the center of the projection plane of the camera. The 
    pos: Vec3,
    // Rotation rotor for the camera.
    // The geometric product of two normalized vectors `a_hat` and `b_hat` in three dimensions
    // (with `x_hat`, `y_hat`, and `z_hat` being the basis vectors) gives:
    // (a_hat)(b_hat) = a_hat*b_hat + a_hat ∧ b_hat
    // = a_x*b_x + a_y*b_y + a_z*b_z
    // + (a_x*b_y - a_y*b_x)*(x_hat ∧ y_hat)
    // + (a_y*b_z - a_z*b_y)*(y_hat ∧ z_hat)
    // + (a_z*b_x - a_x*b_z)*(z_hat ∧ x_hat).
    // 
    // Because three dimensions is a special case where the exterior products can correspond to
    // a unique vector tangent to the bivector, we can alternatively use a rotation of angle
    // `theta` around an axis `a_hat`:
    // = cos(theta / 2)
    // + a_x*sin(theta / 2)
    // + a_y*sin(theta / 2)
    // + a_z*sin(theta / 2),
    // but this requires deciding handedness, if `z_hat ∧ x_hat = y_hat` or if
    // `x_hat ∧ z_hat = y_hat`.
    // 
    // To make this more general, we use the geometric product expansion instead and expose three
    // functions, each with `a_hat` set to a different absolute basis vector, and allow the user to
    // put in an arbitrary `b_hat`. The only hard coded assumptions are that the default camera
    // (with no rotation applied) is rotated to look towards the positive x axis direction, with
    // the y axis to the right, and the z axis is up (reflections are applied to `rot_mat` to make
    // the output image right side up, due to the y-axis being down in typical image formats).
    //rot: Vec4,

    /// Projection matrix used for the direction of the camera.
    /// `proj_mat.0` is the vector perpendicular to the projection plane. It starts at the origin
    /// of the projection rectangle and indicates the direction of the camera.
    /// `proj_mat.1` is a vector parallel to the projection plane and corresponds with the x axis of
    /// the image.
    /// `proj_mat.2` is another vector parallel to the projection plane and corresponds with the y
    /// axis of the image.
    /// `proj_mat.1` and `proj_mat.2` are not necessarily perpendicular to each other, which allows
    /// for skew. All the vectors must be normalized.
    proj_mat: (Vec3, Vec3, Vec3),
    /// Distance of the focal point behind the projection plane.
    focal_len: fi64,
    /// The near cutoff plane, behind which objects will not be drawn. This must be positive or else
    /// assumptions made by rendering functions are not met.
    near_cutoff: fi64,
    /// The far cutoff plane, in front of which objects will not be drawn. This must be positive or
    /// else assumptions made by rendering functions are not met. This is intended to be set at
    /// `fiN::MAX - focal_len` or less to prevent overflow issues.
    far_cutoff: fi64,
    /// Dimensions of the projection rectangle, the subset of the projection plane used for
    /// rendering.
    proj_rect_size: Vec2,
    /// The origin corner of the projection rectangle, relative to the center of the projection
    /// plane where the position is. If at least one of a point's projected coordinates are less
    /// than the corresponding min, it will not fit on the projection rectangle.
    proj_rect_min: Vec2,
    /// Opposite corner of the projection rectangle, relative to the center of the projection
    /// plane where the position is. If at least one of a point's projected coordinates are more
    /// than or equal to the corresponding max, it will not fit on the projection rectangle.
    proj_rect_max: Vec2,
    /// Dimensions of the projection rectangle in pixels or points
    proj_rect_points: (u16, u16),
    /// The length of buffers, equal to the width and height of `rect_dim` multiplied.
    buf_len: usize,
}

impl Cam {
    pub fn new(
        pos: Vec3,
        proj_mat: (Vec3, Vec3, Vec3),
        focal_len: fi64,
        near_cutoff: fi64,
        proj_rect_size: Vec2,
        proj_rect_origin: Vec2,
        proj_rect_points: (u16,u16),
    ) -> Result<Self, CamError> {
        // for completeness
        let buf_len = match usize::try_from((proj_rect_points.0 as u32) * (proj_rect_points.1 as u32)) {
            Ok(x) => x,
            Err(_) => return Err(CamError::BufferOverflow)
        };
        if focal_len == fi64::ZERO {
            return Err(CamError::DegenerateProjection)
        }
        if near_cutoff.is_negative()
            || proj_rect_size.0.is_negative()
            || proj_rect_size.1.is_negative()
            || focal_len.is_negative() {
            return Err(CamError::NegativeDimensions)
        }
        let proj_rect_max = (
            match proj_rect_origin.0.checked_add(proj_rect_size.0) {
                Some(x) => x,
                None => return Err(CamError::ProjectionPlaneOverflow)
            },
            match proj_rect_origin.1.checked_add(proj_rect_size.1) {
                Some(x) => x,
                None => return Err(CamError::ProjectionPlaneOverflow)
            }
        );
        Ok(Cam {
            pos,
            proj_mat,
            focal_len,
            near_cutoff,
            far_cutoff: fi64::MAX - focal_len,
            proj_rect_size,
            proj_rect_min: proj_rect_origin,
            proj_rect_max,
            proj_rect_points,
            buf_len
        })
    }

    /// moves the camera around with respect to the absolute basis vectors.
    pub fn mov_self_abs(&mut self, d_pos: Vec3) {
        self.pos.0 += d_pos.0;
        self.pos.1 += d_pos.1;
        self.pos.2 += d_pos.2;
    }

    // TODO: function that rotates by conversion to three rotors and back

    /// rotates the camera with respect to the absolute basis vectors.
    /// `d_rotation` specifies the roll about the x axis, pitch about the y axis, and yaw about the
    /// z axis with left hand chirality. Specifically, a positive d_rotation.0 rolls left, a
    /// positive d_rotation.1 pitches down, and a positive d_rotation.2 yaws right. It applies these
    /// rotations in the order x,y,z.
    /*pub fn rot_self_abs(&mut self, d_rot: Vec3) {
        let rotx = d_rot.0.cos_sin_pi();
        let roty = d_rot.1.cos_sin_pi();
        let rotz = d_rot.2.cos_sin_pi();
        /*self.rot = (
            rotx.0*roty.0,a
            rotx.1*roty.0,b
            rotx.0*roty.1,c
            rotx.1*roty.1,d
        );*/
        //TODO: is all the stuff truly left hand chirality, x forward, y right, z up?
        self.rot = (
            (rotx.0*roty.0*rotz.0) - (rotx.1*roty.1*rotz.1),
            (rotx.1*roty.0*rotz.0) + (rotx.0*roty.1*rotz.1),
            (rotx.0*roty.1*rotz.0) - (rotx.1*roty.0*rotz.1),
            (rotx.1*roty.1*rotz.0) + (rotx.0*roty.0*rotz.1),
        );
        //update matrix
        Self::update_rot_matrix();
    }*/

    /*//moves the camera around with respect to the direction the camera is pointing. The direction the camera is pointing is the z- axis, the x+ axis is to the right of the camera and the y- axis is upwards of the camera.
    pub fn relative_move_self(&mut self, d_position: (fi32, fi32, fi32)) {
        //rotate the change in position vector according to camera rotation before changing position
        self.position.0 += x;
        self.position.1 += y;
        self.position.2 += z;
    }*/

    /*//rotates the camera with respect to its own rotation, similar to relative_move_self
    pub fn relative_rotate_self(&mut self, d_rotation: (i64, i64, i64)) {
        let (x, y, z) = rot(
            calc_rot_matrix((-self.rotation.0, -self.rotation.1, -self.rotation.2)),
            d_rotation,
        );
        self.rotation.0 += x;
        self.rotation.1 += y;
        self.rotation.2 += z;
        //update matrix
        self.matrix = calc_rot_matrix(self.rotation);
    }*/
/*
    //moves and rotates a point according to the camera's position and rotation, returning the point as it appears relative to the camera if the entire world was moved if the camera where placed at the origin and the entire world is rotated so that the camera points down the z- axis with the x+ axis to its right and y- axis is up
    //note: if the point is more than fi32::MAX away from the camera, overflows will happen
    pub fn mov_rot(&self, p: Vec3) -> Vec3 {
        Self::rot(Self::mov(p))
    }

    //translate camera to origin relative to point
    pub fn mov(&self, p: Vec3) -> Vec3 {
        (
            p.0 - self.pos.0,
            p.1 - self.pos.1,
            p.2 - self.pos.2,
        )
    }

    pub fn rot(&self, p: Vec3) -> Vec3 {
        (
            ((self.matrix.0).0 * p.0) + ((self.matrix.0).1 * p.1) + ((self.matrix.0).2 * p.2),
            ((self.matrix.1).0 * p.0) + ((self.matrix.1).1 * p.1) + ((self.matrix.1).2 * p.2),
            ((self.matrix.2).0 * p.0) + ((self.matrix.2).1 * p.1) + ((self.matrix.2).2 * p.2),
        )
    }*/

    // this calculates the orthogonal matrix corresponding to a rotation by the unit quaternion
    // `self.rotation` and stores it in `self.matrix`. Note that this only works if the matrix is
    // used by left multiplying a column vector.
    /*pub fn update_rot_matrix(&mut self) {
        //a + b*i + c*j + d*k
        let (a,b,c,d) = self.rot;
        self.matrix = (
            (
                a*a + b*b - c*c - d*d,
                (b*c - a*d) << 1,
                (b*d + a*c) << 1,
            ),
            (
                (b*c + a*d) << 1,
                a*a - b*b + c*c - d*d,
                (c*d - a*b) << 1,
            ),
            (
                (b*d - a*c) << 1,
                (c*d + a*b) << 1,
                a*a - b*b - c*c + d*d,
            )
        );
    }*/
/*
    pub fn rot(&self, p: (fi32, fi32, fi32)) -> (fi32, fi32, fi32) {
        (
            ((matrix.0).0 * p.0) + ((matrix.0).1 * p.1) + ((matrix.0).2 * p.2),
            ((matrix.1).0 * p.0) + ((matrix.1).1 * p.1) + ((matrix.1).2 * p.2),
            ((matrix.2).0 * p.0) + ((matrix.2).1 * p.1) + ((matrix.2).2 * p.2),
        )
    }
    */

    pub fn project_point_ni_coords(&self, p: Vec3) -> Option<(fi64, fi64)> {
        // translate
        let p = (p.0 - self.pos.0, p.1 - self.pos.1, p.2 - self.pos.2);

        // find the distance perpendicular to the projection plane
        let comp_x =
            (p.0 * (self.proj_mat.0).0)
            + (p.1 * (self.proj_mat.0).1)
            + (p.2 * (self.proj_mat.0).2);
        if comp_x < self.near_cutoff || comp_x >= self.far_cutoff {
            return None;
        }

        // project y
        let comp_y =
            (p.0 * (self.proj_mat.1).0)
            + (p.1 * (self.proj_mat.1).1)
            + (p.2 * (self.proj_mat.1).2);
        let proj_x = fi64(i128_div_rem(
            (comp_y.0 as i128) * (self.focal_len.0 as i128),
            (comp_x.0 as i128) + (self.focal_len.0 as i128)
        ).0 as i64);
        if proj_x < self.proj_rect_min.0 || proj_x >= self.proj_rect_max.0 {
            return None;
        }

        // project z
        let comp_z =
            (p.0 * (self.proj_mat.2).0)
            + (p.1 * (self.proj_mat.2).1)
            + (p.2 * (self.proj_mat.2).2);
        let proj_y = fi64(i128_div_rem(
            (comp_z.0 as i128) * (self.focal_len.0 as i128),
            (comp_x.0 as i128) + (self.focal_len.0 as i128)
        ).0 as i64);
        if proj_y < self.proj_rect_min.1 || proj_y >= self.proj_rect_max.1 {
            return None;
        }
        Some((proj_x, proj_y))
    }

    pub fn project_point(&self, p: Vec3) -> Option<(u16, u16)> {
        let (x, y) = self.project_point_ni_coords(p)?;

        // project from normint coordinates to point coordinates
        Some((
            ((((x - self.proj_rect_min.0).0 as u128) * (self.proj_rect_points.0 as u128))
                / ((self.proj_rect_size.0).0 as u128)) as u16,
            ((((y - self.proj_rect_min.1).0 as u128) * (self.proj_rect_points.1 as u128))
                / ((self.proj_rect_size.1).0 as u128)) as u16
        ))
    }

    /*pub fn draw_line(&self, img: RgbaImage, line: (Vec3, Vec3)) {
        assert!(img.width() == self.proj_rect_points.0);
        assert!(img.height() == self.proj_rect_points.1);
        let p0 = project_point_ni_coords(line.0);
        let p1 = project_point_ni_coords(line.1);
        //
    }*/
}

impl std::default::Default for Cam {
    fn default() -> Self {
        let tmp = fi64(1 << 32);
        let z = fi64::ZERO;
        let o = fi64::ONE;
        Cam::new(
            (z, z, z),
            (
                (o, z, z),
                (z, o, z),
                (z, z, o),
            ),
            tmp,
            z,
            (tmp * 2, tmp * 2),
            (-tmp, -tmp),
            (0x200, 0x200),
        ).unwrap()
    }
}

fn main() {
    use image::{Rgba, RgbaImage, ImageBuffer};

    let mut img: RgbaImage = ImageBuffer::new(0x200, 0x200);
    /*for pixel in img.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }*/

    let cam = Cam::default();

    let bez_00 = (fi64!(0.0002), fi64!(0.), fi64!(0.));
    let bez_01 = (fi64!(0.002), fi64!(0.001), fi64!(0.0005));
    let bez_10 = (fi64!(0.002), fi64!(0.), fi64!(-0.001));
    let bez_11 = (fi64!(0.002), fi64!(0.001), fi64!(-0.0005));
    let n = 50i64;
    for i0 in 0..n {
        for i1 in 0..n {
            let u1 = (fi64!(1.) / n) * i0;
            let u0 = fi64!(1.) - u1;
            let v1 = (fi64!(1.) / n) * i1;
            let v0 = fi64!(1.) - v1;
            let p = (
                (bez_00.0 * u0 * v0) + (bez_01.0 * u1 * v0) + (bez_10.0 * u0 * v1) + (bez_11.0 * u1 * v1),
                (bez_00.1 * u0 * v0) + (bez_01.1 * u1 * v0) + (bez_10.1 * u0 * v1) + (bez_11.1 * u1 * v1),
                (bez_00.2 * u0 * v0) + (bez_01.2 * u1 * v0) + (bez_10.2 * u0 * v1) + (bez_11.2 * u1 * v1)
            );
            match cam.project_point(p) {
                Some((x, y)) => {
                    img.put_pixel(
                        x as u32,
                        y as u32,
                        Rgba([i0 as u8 * 5, 0, i0 as u8 * 5, 255])
                    );
                }
                None => ()
            }
        }
    }

    /*match cam.project_point((fi64!(0.5), fi64!(0.1), fi64!(0.))) {
        Some((x, y)) => {
            img.put_pixel(
                x,
                y,
                Rgba([0 as u8, 0 as u8, 0 as u8, 255])
            );
        }
        None => ()
    }*/

    img.save("microrender_3d.png").unwrap();
}