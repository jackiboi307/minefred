#[macro_export]
macro_rules! gen_struct {
    (
        $svis:vis $sname:ident $( < $lt:lifetime > )?
        { $($fvis:vis $fname:ident : $t:ty = $e:expr),* $(,)? }
        $cvis:vis $cname:ident ) => {

        $svis struct $sname $( < $lt > )? {
            $(
                $fvis $fname: $t,
            )*
        }
        
        impl $( < $lt > )? $sname $( < $lt > )? {
            $cvis fn $cname() -> Self {
                Self {
                    $(
                        $fname: $e,
                    )*
                }
            }
        }
    }
}

pub use crate::gen_struct;
