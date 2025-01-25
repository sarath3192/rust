use crate::spec::{add_link_args, crt_objects, cvs, Cc, LinkerFlavor, LinkOutputKind, LinkSelfContainedDefault, LinkSelfContainedComponents, PanicStrategy, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    // add ld- and cc-style args
    macro_rules! prepare_args {
        ($($val:expr),+) => {{
            let ld_args = &[$($val),+];
            let cc_args = &[$(concat!("-Wl,", $val)),+];

            let mut ret = TargetOptions::link_args(LinkerFlavor::Unix(Cc::No), ld_args);
            add_link_args(&mut ret, LinkerFlavor::Unix(Cc::Yes), cc_args);
            ret
        }};
    }

    let pre_link_args = prepare_args!(
        "-nostdlib",
        "-dynamic-linker=rom/libld-l4.so"
    );

    let late_link_args = prepare_args!(
        "--whole-archive",
        "-lpthread",
        "--no-whole-archive",
        "--start-group",
        "-lc_be_l4refile",
        "-lc_be_l4re",
        "-lc",
        "-lc_nonshared",
        "-ldl",
        "-l4re-util",
        "-l4re",
        "-ll4util",
        "-ll4sys",
        "-lc_support_misc",
        "-lc_be_sig_noop",
        "-lc_be_socket_noop",
        "-lmount",
        "-lgcc",
        "-lgcc_eh",
        "-lld-l4",
        "--end-group"
    );

    let pre_link_objects_self_contained = crt_objects::new(&[
        (LinkOutputKind::StaticNoPicExe, &["crt1.o", "crti.o", "crtbeginT.o"]),
        (LinkOutputKind::StaticPicExe, &["crt1.p.o", "crti.o", "crtbegin.o"]),
        (LinkOutputKind::DynamicNoPicExe, &["crt1.o", "crti.o", "crtbegin.o"]),
        (LinkOutputKind::DynamicPicExe, &["crt1.s.o", "crti.o", "crtbeginS.o"]),
        (LinkOutputKind::DynamicDylib, &["crti.s.o", "crtbeginS.o"]),
        (LinkOutputKind::StaticDylib, &["crti.s.o", "crtbeginS.o"]),
    ]);

    let post_link_objects_self_contained = crt_objects::new(&[
        (LinkOutputKind::StaticNoPicExe, &["crtendT.o", "crtn.o"]),
        (LinkOutputKind::StaticPicExe, &["crtend.o", "crtn.o"]),
        (LinkOutputKind::DynamicNoPicExe, &["crtend.o", "crtn.o"]),
        (LinkOutputKind::DynamicPicExe, &["crtendS.o", "crtn.o"]),
        (LinkOutputKind::DynamicDylib, &["crtendS.o", "crtn.s.o"]),
        (LinkOutputKind::StaticDylib, &["crtendS.o", "crtn.s.o"]),
    ]);

    TargetOptions {
        os: "l4re".into(),
        env: "uclibc".into(),
        panic_strategy: PanicStrategy::Abort,
//        linker: Some("ld".into()),
        linker_flavor: LinkerFlavor::Unix(Cc::No),
        families: cvs!["unix"],
        pre_link_args,
        late_link_args,
        pre_link_objects_self_contained,
        post_link_objects_self_contained,
        link_self_contained: LinkSelfContainedDefault::WithComponents(LinkSelfContainedComponents::LIBC | LinkSelfContainedComponents::CRT_OBJECTS),
        ..Default::default()
    }
}
