use std::error::Error;

use jni::sys::{jint, jsize, JavaVM};

pub fn init() {
    static ANDROID_INIT: std::sync::Once = std::sync::Once::new();

    fn init_inner() -> Result<(), Box<dyn Error>> {
        let jvm = java_vm()?;
        let env = jvm.attach_current_thread_permanently()?;
        init_verifier(&env)?;
        Ok(())
    }

    ANDROID_INIT.call_once(|| {
        if let Err(e) = init_inner() {
            log::error!("Failed to initialize Android support: {}", e);
        }
    });
}

type JniGetCreatedJavaVms =
    unsafe extern "system" fn(vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint;
const JNI_GET_JAVA_VMS_NAME: &[u8] = b"JNI_GetCreatedJavaVMs";

fn java_vm() -> Result<jni::JavaVM, Box<dyn Error>> {
    // Ideally we would use JNI to get a reference to the JavaVM, but that's not possible since
    // UniFFI uses JNA
    //
    // If we could use JNI, we'd just need to export a function and call it from the Android app:
    // #[export_name = "Java_com_orgname_android_rust_init"]
    // extern "C" fn java_init(env: JNIEnv, _class: JClass, context: JObject, ) -> jboolean {
    //
    // Instead we have to use libloading to get a reference to the JavaVM:
    //
    // https://github.com/mozilla/uniffi-rs/issues/1778#issuecomment-1807979746
    let lib = libloading::os::unix::Library::this();
    let get_created_java_vms: JniGetCreatedJavaVms = unsafe { *lib.get(JNI_GET_JAVA_VMS_NAME)? };

    let mut java_vms: [*mut JavaVM; 1] = [std::ptr::null_mut() as *mut JavaVM];
    let mut vm_count: i32 = 0;

    let ok = unsafe { get_created_java_vms(java_vms.as_mut_ptr(), 1, &mut vm_count) };
    if ok != jni::sys::JNI_OK {
        return Err("Failed to get JavaVM".into());
    }
    if vm_count != 1 {
        return Err(format!("Invalid JavaVM count: {vm_count}").into());
    }

    let jvm = unsafe { jni::JavaVM::from_raw(java_vms[0]) }?;
    Ok(jvm)
}

fn init_verifier(env: &jni::JNIEnv<'_>) -> jni::errors::Result<()> {
    let activity_thread = env
        .call_static_method(
            "android/app/ActivityThread",
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
            &[],
        )?
        .l()?;

    let context = env
        .call_method(
            activity_thread,
            "getApplication",
            "()Landroid/app/Application;",
            &[],
        )?
        .l()?;

    Ok(rustls_platform_verifier::android::init_hosted(
        &env, context,
    )?)
}
