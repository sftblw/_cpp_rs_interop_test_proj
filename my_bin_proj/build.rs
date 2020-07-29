extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("../")
        // 'build_target's are not required
        // for test & demo purpose I wrote it.
        // 님들 build_target 필요없어요 걍 명시적으로 써서 돌아가나 보려고 썼어요
        .build_target("MyLibProjCMakeTargetName")
        .build_target("install")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());

    println!("cargo:rustc-link-lib=static=MyLibProjCMakeTargetName");
    println!("cargo:rustc-link-lib=static=stdc++");
}