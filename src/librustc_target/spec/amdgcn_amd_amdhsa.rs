use crate::spec::abi::Abi;
use crate::spec::{
    LinkerFlavor, LldFlavor, MergeFunctions, PanicStrategy, Target, TargetOptions, TargetResult,
};

pub fn target() -> TargetResult {
    Ok(Target {
        arch: "amdgcn".to_string(),
        data_layout: "e-m:e-i64:64-i128:128-v16:16-v32:32-n16:32:64".to_string(),
        llvm_target: "amdgcn-amd-amdhsa".to_string(),

        target_os: "none".to_string(),
        target_vendor: "amd".to_string(),
        target_env: String::new(),

        linker_flavor: LinkerFlavor::Lld(LldFlavor::Link),

        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),

        options: TargetOptions {
            linker: Some("lld".to_string()),

            cpu: "gfx902".to_string(),

            max_atomic_width: Some(64),

            // Unwinding on CUDA is neither feasible nor useful.
            panic_strategy: PanicStrategy::Abort,

            // We can only generate a HSACO, which is really just an ELF
            dynamic_linking: true,
            executables: false,

            // Avoid using dylib because it contain metadata not supported
            // by LLVM NVPTX backend.
            only_cdylib: false,

            // Let the `ptx-linker` to handle LLVM lowering into MC / assembly.
            obj_is_bitcode: true,

            // Convenient and predicable naming scheme.
            dll_prefix: "".to_string(),
            dll_suffix: ".co".to_string(),
            exe_suffix: ".co".to_string(),

            // Disable MergeFunctions LLVM optimisation pass because it can
            // produce kernel functions that call other kernel functions.
            // This behavior is not supported by AMDGPU.
            merge_functions: MergeFunctions::Disabled,

            // We can't generate anything apart from `extern "amdgpu-kernel"` symbols
            abi_blacklist: vec![
                Abi::Cdecl,
                Abi::Stdcall,
                Abi::Fastcall,
                Abi::Vectorcall,
                Abi::Thiscall,
                Abi::Aapcs,
                Abi::Win64,
                Abi::SysV64,
                Abi::PtxKernel,
                Abi::Msp430Interrupt,
                Abi::X86Interrupt,
                Abi::EfiApi,
            
                // Multiplatform / generic ABIs
                Abi::Rust,
                Abi::C,
                Abi::System,
                Abi::RustIntrinsic,
                Abi::RustCall,
                Abi::PlatformIntrinsic,
                Abi::Unadjusted,
            ],

            ..Default::default()
        },
    })
}
