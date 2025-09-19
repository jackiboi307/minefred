#![allow(dead_code)]

use crate::utils::KeyedSlice;

use sdl2::mouse::MouseState;
use sdl2::rect::Rect;

use std::cmp::min;
use std::collections::HashMap;

// Globals

type PosType    = i16;
type SizeType   = PosType;
type MarginType = SizeType;
type Point      = (PosType, PosType);

// Positioning

#[derive(PartialEq, Clone, Copy)]
pub enum Positioning {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

// Orientation

#[derive(PartialEq, Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical   => Self::Horizontal
        }
    }
}

// Margin

pub struct Margin {
    pub top:    MarginType,
    pub bottom: MarginType,
    pub left:   MarginType,
    pub right:  MarginType,
}

impl Margin {
    pub fn new(
            top: MarginType,
            bottom: MarginType,
            left: MarginType,
            right: MarginType) -> Self {

        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn none() -> Self {
        Self {
            top:    0,
            bottom: 0,
            left:   0,
            right:  0,
        }
    }

    pub fn sides(horizontal: MarginType, vertical: MarginType) -> Self {
        Self {
            top:    vertical,
            bottom: vertical,
            left:   horizontal,
            right:  horizontal,
        }
    }

    pub fn even(amount: MarginType) -> Self {
        Self {
            top:    amount,
            bottom: amount,
            left:   amount,
            right:  amount,
        }
    }

    pub fn sum_from_orientation(&self, orientation: Orientation) -> MarginType {
        match orientation {
            Orientation::Horizontal => self.left + self.right,
            Orientation::Vertical   => self.top + self.bottom
        }
    }
}

// RectElement

pub struct RectElement {
    pub x: PosType,
    pub y: PosType,
    pub width:  SizeType,
    pub height: SizeType,
    pub children: KeyedSlice<RectElement>,
}

pub struct CheckResult {
    pub hovered: bool,
    pub x: PosType,
    pub y: PosType,
}

pub struct SpaceEvenlyArgs {
    pub count: u8,
    pub margin: Margin,
    pub size: Option<SizeType>,
    pub spacing: Option<SizeType>,
    pub orientation: Orientation,
}

impl RectElement {
    fn new(x: PosType, y: PosType, width: SizeType, height: SizeType) -> Self {
        Self {
            x,
            y,
            width,
            height,
            children: KeyedSlice::empty(),
        }
    }

    pub fn screen(width: SizeType, height: SizeType) -> Self {
        Self::new(0, 0, width, height)
    }

    pub fn empty() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        Self::new(
            min(p1.0, p2.0),
            min(p1.1, p2.1),
            (p1.1 - p2.1).abs(),
            (p1.1 - p2.1).abs()
        )
    }

    pub fn new_point(
            &self, x: PosType, y: PosType, position: Positioning) -> Point {

        let mut x = x;
        let mut y = y;

        match position {
            Positioning::TopLeft     => {},
            Positioning::TopRight    => { x = self.width  as PosType - x; },
            Positioning::BottomLeft  => { y = self.height as PosType - y; },
            Positioning::BottomRight => { x = self.width  as PosType - x;
                                          y = self.height as PosType - y; },
            Positioning::Center      => { x = self.width  as PosType / 2 + x;
                                          y = self.height as PosType / 2 + y; }
        }

        x += self.x;
        y += self.y;

        (x, y)
    }

    pub fn new_rect(
            &self, x: PosType, y: PosType,
            width: SizeType, height: SizeType, position: Positioning) -> Self {
        
        let p1 = self.new_point(
            x, y, position
        );
        
        let p2 = self.new_point(
            x + if position == Positioning::Center {
                -width / 2
            } else {
                width
            },

            y + if position == Positioning::Center {
                -height / 2
            } else {
                height
            },

            position
        );

        let p = Self::from_points(p1, p2);
        
        Self::new(p.x, p.y, width, height)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.x.into(),
            self.y.into(),

            self.width  as u32,
            self.height as u32
        )
    }

    pub fn size_from_orientation(&self, orientation: Orientation) -> SizeType {
        match orientation {
            Orientation::Horizontal => self.width,
            Orientation::Vertical => self.height
        }
    }

    pub fn check(&self, mouse: &MouseState) -> CheckResult {
        let mx: PosType = mouse.x().try_into().expect("couldn't convert mouse position");
        let my: PosType = mouse.y().try_into().expect("couldn't convert mouse position");

        let x = mx - self.x;
        let y = my - self.y;

        CheckResult {
            hovered:
                self.x <= mx &&
                mx < self.x + self.width as PosType &&
                self.y <= my &&
                my < self.y + self.height as PosType,

            x, y
        }
    }

    pub fn space_evenly(&self, settings: SpaceEvenlyArgs) -> Vec<Self> {
        // very scary function but seems to do its job.

        let mut size = settings.size.unwrap_or(0);
        let count: i16 = settings.count.try_into().unwrap();

        let total_size = if settings.size.is_some() ^ settings.spacing.is_some() {
            let one_of_them = settings.size.unwrap_or_else(|| settings.spacing.unwrap());
            let one_if_size = if settings.size.is_some() { 1 } else { 0 };

            one_of_them + (
                self.size_from_orientation(settings.orientation) -
                settings.margin.sum_from_orientation(settings.orientation) -
                one_of_them * (count - 1 + one_if_size)
            ) / (count - one_if_size)

        } else if settings.size.is_some() && settings.spacing.is_some() {
            settings.size.unwrap() + settings.spacing.unwrap()

        } else {
            panic!("SpaceEvenlyArgs: spacing and/or size must be specified");
        };

        if settings.spacing.is_some() {
            size = total_size - settings.spacing.unwrap()
        }

        let (start_x, start_y) =
            if settings.size.is_some() ^ settings.spacing.is_some() {
                (
                    settings.margin.left,
                    settings.margin.top
                )

            } else {
                let spacing = total_size - size;
                let extra_margin = if settings.orientation == Orientation::Horizontal {
                        self.width
                    } else {
                        self.height
                    } / 2 - (total_size * count - spacing) / 2
                    - if settings.orientation == Orientation::Horizontal {
                        settings.margin.left
                    } else {
                        settings.margin.top
                    };

                (
                    settings.margin.left +
                        if settings.orientation == Orientation::Horizontal
                            { extra_margin } else { 0 },
                    settings.margin.top +
                        if settings.orientation == Orientation::Vertical
                            { extra_margin } else { 0 }
                )
            };

        let mut arr = Vec::with_capacity(count.try_into().unwrap());

        for i in 0..count {
            arr.push(self.new_rect(
                start_x + if settings.orientation == Orientation::Horizontal
                    { total_size } else { 0 } * i as i16,
                start_y + if settings.orientation == Orientation::Vertical
                    { total_size } else { 0 } * i as i16,

                if settings.orientation == Orientation::Vertical {
                    self.width - settings.margin.sum_from_orientation(
                        settings.orientation.opposite())
                } else { size },

                if settings.orientation == Orientation::Horizontal {
                    self.height - settings.margin.sum_from_orientation(
                        settings.orientation.opposite())
                } else { size },

                Positioning::TopLeft
            ));
        }

        arr
    }
}
