// @generated
/// Implement `DataProvider<FullCompositionExclusionV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_comp_ex_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_COMP_EX_V1: &'static <icu_properties::provider::FullCompositionExclusionV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"@\x03\0\0B\x03\0\0C\x03\0\0E\x03\0\0t\x03\0\0u\x03\0\0~\x03\0\0\x7F\x03\0\0\x87\x03\0\0\x88\x03\0\0X\t\0\0`\t\0\0\xDC\t\0\0\xDE\t\0\0\xDF\t\0\0\xE0\t\0\x003\n\0\x004\n\0\x006\n\0\x007\n\0\0Y\n\0\0\\\n\0\0^\n\0\0_\n\0\0\\\x0B\0\0^\x0B\0\0C\x0F\0\0D\x0F\0\0M\x0F\0\0N\x0F\0\0R\x0F\0\0S\x0F\0\0W\x0F\0\0X\x0F\0\0\\\x0F\0\0]\x0F\0\0i\x0F\0\0j\x0F\0\0s\x0F\0\0t\x0F\0\0u\x0F\0\0w\x0F\0\0x\x0F\0\0y\x0F\0\0\x81\x0F\0\0\x82\x0F\0\0\x93\x0F\0\0\x94\x0F\0\0\x9D\x0F\0\0\x9E\x0F\0\0\xA2\x0F\0\0\xA3\x0F\0\0\xA7\x0F\0\0\xA8\x0F\0\0\xAC\x0F\0\0\xAD\x0F\0\0\xB9\x0F\0\0\xBA\x0F\0\0q\x1F\0\0r\x1F\0\0s\x1F\0\0t\x1F\0\0u\x1F\0\0v\x1F\0\0w\x1F\0\0x\x1F\0\0y\x1F\0\0z\x1F\0\0{\x1F\0\0|\x1F\0\0}\x1F\0\0~\x1F\0\0\xBB\x1F\0\0\xBC\x1F\0\0\xBE\x1F\0\0\xBF\x1F\0\0\xC9\x1F\0\0\xCA\x1F\0\0\xCB\x1F\0\0\xCC\x1F\0\0\xD3\x1F\0\0\xD4\x1F\0\0\xDB\x1F\0\0\xDC\x1F\0\0\xE3\x1F\0\0\xE4\x1F\0\0\xEB\x1F\0\0\xEC\x1F\0\0\xEE\x1F\0\0\xF0\x1F\0\0\xF9\x1F\0\0\xFA\x1F\0\0\xFB\x1F\0\0\xFC\x1F\0\0\xFD\x1F\0\0\xFE\x1F\0\0\0 \0\0\x02 \0\0&!\0\0'!\0\0*!\0\0,!\0\0)#\0\0+#\0\0\xDC*\0\0\xDD*\0\0\0\xF9\0\0\x0E\xFA\0\0\x10\xFA\0\0\x11\xFA\0\0\x12\xFA\0\0\x13\xFA\0\0\x15\xFA\0\0\x1F\xFA\0\0 \xFA\0\0!\xFA\0\0\"\xFA\0\0#\xFA\0\0%\xFA\0\0'\xFA\0\0*\xFA\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\x1D\xFB\0\0\x1E\xFB\0\0\x1F\xFB\0\0 \xFB\0\0*\xFB\0\x007\xFB\0\08\xFB\0\0=\xFB\0\0>\xFB\0\0?\xFB\0\0@\xFB\0\0B\xFB\0\0C\xFB\0\0E\xFB\0\0F\xFB\0\0O\xFB\0\0^\xD1\x01\0e\xD1\x01\0\xBB\xD1\x01\0\xC1\xD1\x01\0\0\xF8\x02\0\x1E\xFA\x02\0") }, 1120u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::FullCompositionExclusionV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::FullCompositionExclusionV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_COMP_EX_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::FullCompositionExclusionV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
