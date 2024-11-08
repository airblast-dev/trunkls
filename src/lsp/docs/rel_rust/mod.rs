use crate::{bulk_struct, load_md};

bulk_struct! {
    Href,
    DataTargetName,
    DataBin,
    DataType,
    DataCargoFeatures,
    DataCargoNoDefaultFeatures,
    DataCargoAllFeatures,
    DataWasmOpt,
    DataWasmOptParams,
    DataKeepDebug,
    DataNoDemangle,
    DataReferenceTypes,
    DataWeakRefs,
    DataTypeScript,
    DataBindgenTarget,
    DataLoaderShim,
    DataCrossOrigin,
    DataWasmNoImport,
    DataWasmImportName
}
load_md! {Href, "href", "href"}
load_md! {DataTargetName, "data_target_name", "data-target-name"}
load_md! {DataBin, "data_bin", "data-bin"}
load_md! {DataType, "data_type", "data-type"}
load_md! {DataCargoFeatures, "data_cargo_features", "data-cargo-features"}
load_md! {DataCargoNoDefaultFeatures, "data_cargo_no_default_features","data-cargo-no-default-features"}
load_md! {DataCargoAllFeatures, "data_cargo_all_features", "data-cargo-all-features"}
load_md! {DataWasmOpt, "data_wasm_opt", "data-wasm-opt"}
load_md! {DataWasmOptParams, "data_wasm_opt_params", "data-wasm-opt-params"}
load_md! {DataKeepDebug, "data_keep_debug", "data-keep-debug"}
load_md! {DataNoDemangle, "data_no_demangle", "data-no-demangle"}
load_md! {DataReferenceTypes, "data_reference_types", "data-reference-types"}
load_md! {DataWeakRefs, "data_weak_refs", "data-weak-refs"}
load_md! {DataTypeScript, "data_typescript", "data-typescript"}
load_md! {DataBindgenTarget, "data_bindgen_targets", "data-bindgen-targets"}
load_md! {DataLoaderShim, "data_loader_shim", "data-loader-shim"}
load_md! {DataCrossOrigin, "data_cross_origin", "data-cross-origin"}
load_md! {DataWasmNoImport, "data_wasm_no_import", "data-wasm-no-import"}
load_md! {DataWasmImportName, "data_wasm_import_name", "data-wasm-import-name"}
