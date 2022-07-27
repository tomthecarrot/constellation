mod background;

#[cfg(feature = "c_api")]
mod c_api {
    // This generates the C header file for the bindings. See safer-ffi's guide.
    #[safer_ffi::cfg_headers]
    #[test]
    fn generate_headers() -> ::std::io::Result<()> {
        let builder = ::safer_ffi::headers::builder();
        if ::std::env::var("HEADERS_TO_STDOUT")
            .ok()
            .map_or(false, |it| it == "1")
        {
            builder.to_writer(::std::io::stdout()).generate()
        } else {
            builder.to_file(&"generated.h".to_string())?.generate()
        }
    }
}
