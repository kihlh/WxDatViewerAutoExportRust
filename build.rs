extern crate winres;
#[cfg(windows)]
use winres::WindowsResource;

fn main() {
    static_vcruntime::metabuild();

    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
    <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
    </assembly>
    "#,
    );
    #[cfg(windows)]
    {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("src/icon/ICON1.ico")
            .set_icon_with_id("src/icon/ICON1.ico", "appicon")
            .compile()
            .expect("set icon error");
    }

    // Build::new().compile("app.rc").unwrap();

    // cc::Build::new()
    // .file("src/hello.c")
    // .compile("hello") ;
    // pkg_config::Config::new().probe("libWxIkunPlus").unwrap();
}
