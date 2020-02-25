#[cfg(test)]
mod test {
    #[repr(C)]
    struct msdfgen_SignedDistance {
        distance: f64,
        dot: f64,
    }

    extern "C" {
        #[link_name = "\u{1}_ZN7msdfgen14SignedDistance8INFINITEE"]
        static msdfgen_SignedDistance_INFINITE: msdfgen_SignedDistance;
    }

    #[test]
    fn linking() {
        let infinite = unsafe { &msdfgen_SignedDistance_INFINITE };

        assert_eq!(infinite.distance, -1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0);
        assert_eq!(infinite.dot, 1.0);
    }
}
