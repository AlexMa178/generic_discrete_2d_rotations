use generic_discrete_2d_rotations::{Rot, RotDir, RotFrom};


#[test]
fn from_signs() {

    assert_eq!(Rot::from_signs::<4>(RotFrom::PosY, RotDir::Clockwise, 0   .cmp(&0), 1   .cmp(&0)), Some(Rot::R4_0  ));
    assert_eq!(Rot::from_signs::<4>(RotFrom::PosY, RotDir::Clockwise, 1   .cmp(&0), 0   .cmp(&0)), Some(Rot::R4_90 ));
    assert_eq!(Rot::from_signs::<4>(RotFrom::PosY, RotDir::Clockwise, 0   .cmp(&0), (-1).cmp(&0)), Some(Rot::R4_180));
    assert_eq!(Rot::from_signs::<4>(RotFrom::PosY, RotDir::Clockwise, (-1).cmp(&0), 0   .cmp(&0)), Some(Rot::R4_270));

}

#[test]
fn from_vector() {

    assert_eq!(Rot::from_vector::<4>(RotFrom::PosY, RotDir::Clockwise, 0., 0.), None);
    assert_eq!(Rot::from_vector::<4>(RotFrom::PosY, RotDir::Clockwise, 0., 1.), Some(Rot::R4_0));

}

#[test]
fn angle_to() {

    assert_eq!(RotFrom::PosY.angle_to(RotFrom::PosY, RotDir::Clockwise       ), Rot::R4_0  );
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::PosX, RotDir::Clockwise       ), Rot::R4_90 );
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::NegY, RotDir::Clockwise       ), Rot::R4_180);
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::NegX, RotDir::Clockwise       ), Rot::R4_270);
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::PosY, RotDir::CounterClockwise), Rot::R4_0  );
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::NegX, RotDir::CounterClockwise), Rot::R4_90);
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::NegY, RotDir::CounterClockwise), Rot::R4_180);
    assert_eq!(RotFrom::PosY.angle_to(RotFrom::PosX, RotDir::CounterClockwise), Rot::R4_270);

}

#[test]
fn change_relative_to() {

    assert_eq!(Rot::R4_0.change_relative_to(RotFrom::PosY, RotDir::Clockwise, RotFrom::PosX, RotDir::CounterClockwise), Rot::R4_90);

}