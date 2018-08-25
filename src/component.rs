use field::icon::Icons;
use field::id::Id;
use field::license::License;
use field::name::Name;
use field::pkg_name::PkgName;
use field::summary::Summary;

pub trait Component {
    const ATTRIBUTE: &'static str;

//     fn id(&self) -> &Id;
//     fn name(&self) -> &Name;
//     fn summary(&self) -> &Summary;
//     fn pkg_name(&self) -> &PkgName;

//     fn license(&self) -> Option<&License> {
//         None
//     }

//     fn metadata_license(&self) -> Option<&License> {
//         None
//     }

//     fn icons(&self) -> Icons {
//         Icons::new(&[])
//     }
}

pub struct DesktopApp;

impl Component for DesktopApp {
    const ATTRIBUTE: &'static str = "desktop";
}

// TODO: DesktopApp
// TODO: ConsoleApp
// TODO: WebApp
// TODO: Service
// TODO: Addon
// TODO: Font
// TODO: Codec
// TODO: InputMethod
// TODO: Firmware
// TODO: Driver
// TODO: Localization
